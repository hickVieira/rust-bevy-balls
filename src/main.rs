pub mod consts;
pub mod types;
// pub mod pbd;

mod impulse {
    use crate::types::*;

    pub async fn main() {
        let world = &mut world::World {
            balls: vec![],
            frame_time: 0.0,
            physics_time: 0.0,
            system_energy: 0.0,
        };

        loop {
            world.draw().await;
            world.solve_input().await;
            world.solve_physics();
        }
    }
}

#[macroquad::main("physics")]
async fn main() {
    macroquad::miniquad::window::set_fullscreen(true);
    // pbd::main().await;
    impulse::main().await;
}
