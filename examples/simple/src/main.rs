use material_ppm::PpmScreen;
use material_ui::App;

fn main() {
    let mut buf = vec![0; 800 * 600 * 3];
    let mut display = PpmScreen::new((800, 600), &mut buf);

    let mut app = App::new();
    app.draw(&mut display, |ui| {
        ui.button("Hello, World!");
        ui.button("Bye, World!");
    });
}
