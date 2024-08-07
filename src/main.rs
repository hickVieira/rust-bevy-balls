use macroquad::miniquad::window;

mod pbd;
mod AxisChain;

#[macroquad::main("pbd")]
async fn main() {
    window::set_fullscreen(true);
    pbd::main().await;
}
