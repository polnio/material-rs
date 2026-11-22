use material_ui::DrawTarget;
use material_ui::prelude::*;
use ouroboros::self_referencing;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct WinitScreen<'a> {
    on_draw_requested: Box<dyn FnMut(&mut WinitScreenDisplay) + 'a>,
    ctx: Option<WinitScreenCtx>,
}

struct WinitScreenCtx {
    window: WinitScreenWindow,
    cursor_pos: Option<Point>,
    cursor_state: CursorState,
}

#[self_referencing]
struct WinitScreenWindow {
    winit: Window,
    #[borrows(winit)]
    #[covariant]
    surface: softbuffer::Surface<&'this Window, &'this Window>,
}

pub struct WinitScreenDisplay<'a> {
    ctx: &'a mut WinitScreenCtx,
}

impl<'a> DrawTarget for WinitScreenDisplay<'a> {
    fn size(&self) -> Size {
        let size = self.ctx.window.borrow_winit().inner_size();
        Size::new(size.width, size.height)
    }

    fn cursor_pos(&self) -> Option<Point> {
        self.ctx.cursor_pos
    }

    fn cursor_state(&self) -> CursorState {
        self.ctx.cursor_state
    }

    fn clear(&mut self, color: Color) {
        self.ctx.window.with_surface_mut(|surface| {
            let mut buffer = surface.buffer_mut().unwrap();
            buffer.fill(color.to_argb());
        });
    }

    fn draw_pixel(&mut self, point: impl Into<Point>, color: Color) {
        self.ctx.window.with_surface_mut(|surface| {
            let mut buffer = surface.buffer_mut().unwrap();

            let point = point.into();
            let x = point.x();
            let y = point.y();
            let w = buffer.width().get();
            let h = buffer.height().get();
            if x < 0 || x as u32 > w || y < 0 || y as u32 > h {
                return;
            }
            let Some(pixel) = buffer.get_mut(y as usize * w as usize + x as usize) else {
                return;
            };
            let [_, r, g, b] = pixel.to_be_bytes();
            let [r, g, b] = material_ui::util::blend_color(color, [r, g, b]);
            *pixel = u32::from_be_bytes([0, r, g, b]);
        });
    }

    fn flush(&mut self) {
        self.ctx.window.with_surface_mut(|surface| {
            surface.buffer_mut().unwrap().present().unwrap();
        })
    }
}

impl<'a> ApplicationHandler for WinitScreen<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create Winit window");
        let window = WinitScreenWindow::new(window, |window| {
            let ctx =
                softbuffer::Context::new(window).expect("Failed to create softbuffer context");
            softbuffer::Surface::new(&ctx, window).expect("Failed to create surface")
        });
        self.ctx = Some(WinitScreenCtx {
            window,
            cursor_pos: None,
            cursor_state: CursorState::empty(),
        })
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(ctx) = &mut self.ctx else {
            return;
        };
        if window_id != ctx.window.borrow_winit().id() {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                ctx.cursor_pos = Some(Point::new(position.x as i32, position.y as i32));
                ctx.window.borrow_winit().request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => match (button, state) {
                (MouseButton::Left, ElementState::Pressed) => {
                    ctx.cursor_state.set(CursorState::LEFT_CLICK, true);
                    ctx.window.borrow_winit().request_redraw();
                }
                (MouseButton::Left, ElementState::Released) => {
                    ctx.cursor_state.set(CursorState::LEFT_CLICK, false);
                    ctx.window.borrow_winit().request_redraw();
                }
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                let size = ctx.window.borrow_winit().inner_size();
                ctx.window.with_surface_mut(|surface| {
                    surface
                        .resize(
                            size.width.try_into().unwrap(),
                            size.height.try_into().unwrap(),
                        )
                        .unwrap();
                });
                let mut handler = WinitScreenDisplay { ctx };
                (self.on_draw_requested)(&mut handler);
            }
            _ => {}
        }
    }
}

pub fn create_window(f: impl FnMut(&mut WinitScreenDisplay)) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = WinitScreen {
        on_draw_requested: Box::new(f),
        ctx: None,
    };
    event_loop.run_app(&mut app).unwrap();
}
