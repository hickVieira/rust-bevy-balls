use macroquad::miniquad::window;

mod pbd;
mod vel;

#[macroquad::main("pbd")]
async fn main() {
    window::set_fullscreen(true);
    // pbd::main().await;
    vel::main().await;
}
