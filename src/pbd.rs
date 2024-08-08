use core::f32;
use std::collections::HashSet;

use macroquad::prelude::*;

use crate::AxisChain::*;

const RADIUS: f32 = 20.0;

struct Particle {
    pub id: usize,
    pub pos: Vec2,
    pub post_last: Vec2,
    pub accel: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Particle {
    fn new(id: usize, pos: Vec2, mass: f32, radius: f32) -> Self {
        Self {
            id,
            pos,
            post_last: pos,
            accel: Vec2::new(0.0, 0.0),
            mass,
            radius,
        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, screen_height() - self.pos.y, self.radius, BLACK);
        draw_text(
            format!("{}", self.id).as_str(),
            self.pos.x - self.radius / 2.0,
            screen_height() - self.pos.y + self.radius / 2.0,
            20.0,
            GREEN,
        );
    }

    fn integrate(&mut self, dt: f32) {
        let vel = self.pos - self.post_last;
        self.post_last = self.pos;
        self.pos = self.pos + vel + self.accel * dt * dt;
        self.accel = Vec2::new(0.0, 0.0);
    }

    fn displace(&mut self, delta: Vec2, smooth: f32) {
        self.pos += delta;
        self.post_last += delta * smooth;
    }

    fn apply_acceleration(&mut self, accel: Vec2) {
        self.accel += accel;
    }

    fn apply_force(&mut self, force: Vec2) {
        self.accel += force / self.mass;
    }
}

struct World {
    particles: Vec<Particle>,
    frame_time: f32,
    physics_time: f32,
    system_energy: f32,
}

impl World {
    async fn draw(&mut self) {
        let timer = std::time::Instant::now();
        clear_background(WHITE);
        for p in self.particles.iter() {
            p.draw();
        }
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("Press SPACE to add particles", 20.0, 40.0, 30.0, DARKGRAY);
        draw_text(
            &format!("Frame time: {}ms", self.frame_time),
            20.0,
            80.0,
            30.0,
            DARKGRAY,
        );
        draw_text(
            &format!("Physics time: {}ms", self.physics_time),
            20.0,
            100.0,
            30.0,
            DARKGRAY,
        );
        draw_text(
            &format!("Particles: {}", self.particles.len()),
            20.0,
            120.0,
            30.0,
            DARKGRAY,
        );
        draw_text(
            &format!("System energy: {}", self.system_energy),
            20.0,
            140.0,
            30.0,
            DARKGRAY,
        );
        next_frame().await;
        self.frame_time = timer.elapsed().as_secs_f32();
    }

    fn solve_physics(&mut self) {
        let timer = std::time::Instant::now();
        // get system energy
        self.system_energy = {
            let mut sum = 0.0;
            for p in self.particles.iter() {
                sum += (p.pos - p.post_last).length();
            }
            sum / (self.particles.len() as f32)
        };

        // add gravity
        for p in self.particles.iter_mut() {
            p.apply_acceleration(Vec2::new(0.0, -9.8));
        }

        // integrate
        for p in self.particles.iter_mut() {
            p.integrate(0.1);
        }

        // wall collisions
        for p in self.particles.iter_mut() {
            let dist_y = p.pos.y - p.radius;
            if dist_y < 0.0 {
                p.displace(Vec2::new(0.0, -dist_y), 0.0);
            }

            let dist_x = p.pos.x - p.radius;
            if dist_x < 0.0 {
                p.displace(Vec2::new(-dist_x, 0.0), 0.0);
            }

            let dist_x = p.pos.x + p.radius;
            if dist_x > screen_width() {
                p.displace(Vec2::new(screen_width() - dist_x, 0.0), 0.0);
            }
        }

        // sort axis
        // let mut x_axis = vec![(0, 0.0); self.particles.len()];
        // let mut y_axis = vec![(0, 0.0); self.particles.len()];
        // for i in 0..self.particles.len() {
        //     x_axis[i] = (i, self.particles[i].pos.x);
        //     y_axis[i] = (i, self.particles[i].pos.y);
        // }
        // x_axis.sort_by(|a, b| a.1.total_cmp(&b.1));
        // y_axis.sort_by(|a, b| a.1.total_cmp(&b.1));

        // let mut x_map = vec![0; self.particles.len()];
        // let mut y_map = vec![0; self.particles.len()];
        // for i in 0..self.particles.len() {
        //     x_map[x_axis[i].0] = i;
        //     y_map[y_axis[i].0] = i;
        // }

        // particle collisions
        // let mut displacements: Vec<Vec2> = vec![Vec2::ZERO; self.particles.len()];

        // for i in 0..self.particles.len() {
        //     let p = &self.particles[i];
        //     let index = x_map[i];
        // }

        let mut x_axis = AxisProjection::new(self.particles.len());
        let mut y_axis = AxisProjection::new(self.particles.len());
        for i in 0..self.particles.len() {
            x_axis.set(i, self.particles[i].pos.x);
            y_axis.set(i, self.particles[i].pos.y);
        }
        x_axis.build(RADIUS * 2.0);
        y_axis.build(RADIUS * 2.0);

        // print!("\nx : ");
        // for i in 0..x_axis.length {
        //     print!("{} ", x_axis.sorted[i].0);
        // }
        // print!("\n");
        // for i in 0..x_axis.length {
        //     print!("({},{}) ", x_axis.ranges[i].0, x_axis.ranges[i].1);
        // }
        // print!("\ny : ");
        // for i in 0..y_axis.length {
        //     print!("{} ", y_axis.sorted[i].0);
        // }
        // print!("\n");
        // for i in 0..y_axis.length {
        //     print!("({},{}) ", y_axis.ranges[i].0, y_axis.ranges[i].1);
        // }
        // print!("\n");

        // particle collisions
        let mut displacements: Vec<Vec2> = vec![Vec2::ZERO; self.particles.len()];
        // let mut pairs: Vec<(usize, usize)> = vec![];

        // for i in 0..self.particles.len() {
        //     let proj_x_index = x_axis.src_to_proj(i);
        //     let proj_y_index = y_axis.src_to_proj(i);

        //     let x_range = x_axis.get_range(proj_x_index);
        //     let y_range = y_axis.get_range(proj_y_index);

        //     // let x_range =

        //     for x in x_range.0..x_range.1 {
        //         let src_x_index = x_axis.proj_to_src(x);
        //         if i == src_x_index {
        //             continue;
        //         }

        //         pairs.push((i, src_x_index));

        //         // for y in y_range.0..y_range.1 {
        //         //     let src_y_index = y_axis.proj_to_src(y);
        //         //     if i == src_y_index {
        //         //         continue;
        //         //     }
        //         //     pairs.push((i, src_y_index));
        //         // }
        //     }
        // }

        // for i in pairs.iter() {
        //     println!("{} ({} {})", pairs.len(), i.0, i.1);
        // }

        // collide pairs
        // for p in pairs.iter() {
        //     let p0 = &self.particles[p.0];
        //     let p1 = &self.particles[p.1];
        //     let delta = p0.pos - p1.pos;
        //     let dist = delta.length() + f32::EPSILON;
        //     let diff = p0.radius + p1.radius - dist;
        //     if diff > 0.0 {
        //         let normal = delta / dist;
        //         displacements[p.0] += normal * diff * 0.5;
        //         displacements[p.1] -= normal * diff * 0.5;
        //     }
        // }

        // particle collisions
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }
                let p0 = &self.particles[i];
                let p1 = &self.particles[j];
                let delta = p0.pos - p1.pos;
                let dist = delta.length() + f32::EPSILON;
                let diff = p0.radius + p1.radius - dist;
                if diff > 0.0 {
                    let normal = delta / dist;
                    displacements[i] += normal * diff * 0.5;
                    displacements[j] -= normal * diff * 0.5;
                }
            }
        }

        // apply displacements
        for i in 0..self.particles.len() {
            self.particles[i].displace(displacements[i], 0.25);
        }
        self.physics_time = timer.elapsed().as_secs_f32();
    }

    async fn solve_input(&mut self) {
        if is_key_down(KeyCode::Space) {
            for _ in 0..1 {
                self.particles.push(Particle::new(
                    self.particles.len(),
                    Vec2::new(
                        rand::gen_range(0, screen_width() as u32) as f32,
                        rand::gen_range(0, screen_height() as u32) as f32,
                    ),
                    1.0,
                    RADIUS,
                ));
            }
        }
    }
}

pub async fn main() {
    let world = &mut World {
        particles: vec![],
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
