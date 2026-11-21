use material_ui::App;

fn main() {
    // let mut display = WinitScreen::new((800, 600), "Simple window");
    material_winit::create_window(|display| {
        let mut app = App::new();
        app.draw(display, |ui| {
            ui.button("Hello, World!");
            ui.button("Bye, World!");
        });
    });
}
