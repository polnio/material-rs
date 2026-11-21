use material_ui::App;

fn main() {
    material_winit::create_window(|display| {
        let mut app = App::new();
        app.draw(display, |ui| {
            if ui.button("Hello, World!").clicked() {
                println!("Hi!");
            }
            ui.button("Bye, World!");
        });
    });
}
