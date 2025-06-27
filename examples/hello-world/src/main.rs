use material_sdl2_renderer::SDL2Renderer;
use material_terminal_renderer::TerminalRenderer;

fn main() {
    let content = "Hello, World!";

    let mut args = std::env::args();
    let _ = args.next();
    let renderer_id = args.next();
    let renderer_id = renderer_id.as_deref().unwrap_or("terminal");
    match renderer_id {
        "sdl2" => {
            let mut renderer = SDL2Renderer::new();
            renderer.render(content);
        }
        "terminal" => {
            let mut renderer = TerminalRenderer::new();
            renderer.render(content);
        }
        _ => {
            eprintln!("Unknown renderer: {}", renderer_id);
            std::process::exit(1);
        }
    }
}
