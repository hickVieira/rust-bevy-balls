use macroquad::prelude::*;

pub const RADIUS: f32 = 10.0;
pub const PHY_ITERATIONS: u32 = 4;
pub const PHY_DT: f32 = 1.0 / 60.0;
pub const PHY_SDT: f32 = PHY_DT / PHY_ITERATIONS as f32;
pub const GRAVITY: Vec2 = Vec2::new(0.0, -9.8);
pub const ELASTICITY: f32 = 0.5;
pub const PI: f32 = std::f32::consts::PI;
