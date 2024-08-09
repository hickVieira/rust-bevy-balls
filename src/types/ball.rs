use macroquad::prelude::*;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new(pos: Vec2, mass: f32, radius: f32, vel: Vec2) -> Self {
        Self {
            pos,
            vel,
            mass,
            radius,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, screen_height() - self.pos.y, self.radius, BLACK);
        // draw_text(
        //     format!("{}", self.id).as_str(),
        //     self.pos.x - self.radius / 2.0,
        //     screen_height() - self.pos.y + self.radius / 2.0,
        //     20.0,
        //     GREEN,
        // );
    }

    pub fn integrate(&mut self, dt: f32) {
        self.pos = self.pos + self.vel * dt;
    }

    pub fn displace(&mut self, delta: Vec2) {
        // let pos_old = self.pos;
        self.pos = self.pos + delta;
        // self.vel = self.vel + (self.pos - pos_old);
    }

    pub fn apply_acceleration(&mut self, accel: Vec2) {
        self.vel = self.vel + accel;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.vel = self.vel + force / self.mass;
    }

    pub fn apply_velocity(&mut self, vel: Vec2) {
        self.vel = self.vel + vel;
    }
}
