use material_ui::App;

fn main() {
    let mut app = App::new();
    let mut count = 0usize;

    material_winit::create_window(|display| {
        app.draw(display, |ui| {
            if ui.button("Hello, World!").clicked() {
                count += 1;
            }
            ui.label(&format!("Clicked {count} times"));
        });
    });
}
