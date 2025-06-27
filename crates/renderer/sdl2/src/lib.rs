use font_kit::family_name::FamilyName;
use material_core::geometry::{Color, Point, Rect};
use material_core::renderer::{Renderer, RendererInner};
use material_core::widgets::{IntoWidget, Widget};
use ouroboros::self_referencing;
use sdl2::event::Event;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;

trait ToSDLExt {
    type Output;
    fn to_sdl(self) -> Self::Output;
}
impl ToSDLExt for Color {
    type Output = sdl2::pixels::Color;
    fn to_sdl(self) -> Self::Output {
        Self::Output {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
impl ToSDLExt for Rect {
    type Output = sdl2::rect::Rect;
    fn to_sdl(self) -> Self::Output {
        Self::Output::new(
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        )
    }
}

#[self_referencing]
struct SDL2RendererTTF {
    ttf_context: sdl2::ttf::Sdl2TtfContext,
    #[borrows(ttf_context)]
    #[covariant]
    font: Font<'this, 'static>,
}

pub struct SDL2Renderer {
    inner: RendererInner,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    ttf: SDL2RendererTTF,
}
impl SDL2Renderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL");
        let ttf_context = sdl2::ttf::init().expect("Failed to initialize SDL TTF");

        let video_subsystem = sdl_context
            .video()
            .expect("Failed to create SDL video subsystem");
        let window = video_subsystem
            .window("SDL2 Material", 800, 600)
            .build()
            .expect("Failed to create SDL window");

        let canvas = window
            .into_canvas()
            .build()
            .expect("Failed to create SDL canvas");
        let event_pump = sdl_context.event_pump().unwrap();

        let font_kit::handle::Handle::Path {
            path: font_path, ..
        } = font_kit::source::SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Default::default())
            .expect("Failed to load font")
        else {
            panic!("Font is not a path");
        };

        let ttf = SDL2RendererTTF::new(ttf_context, |ttf_context| {
            ttf_context
                .load_font(font_path, 32)
                .expect("Failed to load font")
        });

        let mut inner = RendererInner::new();
        inner.surface_size = (800, 600);

        SDL2Renderer {
            inner,
            canvas,
            event_pump,
            ttf,
        }
    }
    pub fn render(&mut self, widget: impl IntoWidget) {
        Renderer::render(self, widget);
    }
}

impl Renderer for SDL2Renderer {
    fn inner(&mut self) -> &mut RendererInner {
        &mut self.inner
    }
    fn text_size(&self, text: &str) -> (u32, u32) {
        self.ttf.borrow_font().size_of(text).unwrap()
    }
    fn draw_text(&mut self, text: &str, pos: Point, color: Color) {
        let texture_creator = self.canvas.texture_creator();
        let surface = self
            .ttf
            .borrow_font()
            .render(text)
            .solid(color.to_sdl())
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, width, height);
        self.canvas.copy(&texture, None, target).unwrap();
    }

    fn draw_rect(&mut self, rect: Rect, color: Color) {
        let c = self.canvas.draw_color();
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas.fill_rect(rect.to_sdl()).unwrap();
        self.canvas.set_draw_color(c);
    }

    fn render(&mut self, widget: impl IntoWidget) {
        let mut widget = widget.into_widget();

        self.canvas.set_draw_color(Color::BLACK.to_sdl());
        self.canvas.clear();
        self.canvas.present();
        self.compute_layout(&mut widget);
        'running: loop {
            self.canvas.clear();
            widget.render(self);
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }
            self.canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
