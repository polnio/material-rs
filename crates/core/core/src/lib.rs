pub mod geometry;
mod renderer;
pub mod widgets;

pub use renderer::Renderer;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
