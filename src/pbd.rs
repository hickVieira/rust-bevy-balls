use std::collections::HashSet;

use macroquad::prelude::*;

use crate::AxisChain::*;

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
    }

    fn integrate(&mut self, dt: f32) {
        let vel = self.pos - self.post_last;
        self.post_last = self.pos;
        self.pos = self.pos + vel + self.accel * dt * dt;
        self.accel = Vec2::new(0.0, 0.0);
    }

    fn displace(&mut self, delta: Vec2, t: f32) {
        self.pos += delta * t;
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
    dt: f32,
}

impl World {
    async fn draw(&mut self) {
        let timer = std::time::Instant::now();
        clear_background(WHITE);
        for p in self.particles.iter() {
            p.draw();
        }
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("Press SPACE to add particles", 20.0, 60.0, 30.0, DARKGRAY);
        draw_text(
            &format!("Frame time: {}ms", self.dt),
            20.0,
            100.0,
            30.0,
            DARKGRAY,
        );
        draw_text(
            &format!("Particles: {}", self.particles.len()),
            20.0,
            140.0,
            30.0,
            DARKGRAY,
        );
        next_frame().await;
        self.dt = timer.elapsed().as_secs_f32();
    }

    fn solve_physics(&mut self) {
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
                p.displace(Vec2::new(0.0, -dist_y), 1.0);
            }

            let dist_x = p.pos.x - p.radius;
            if dist_x < 0.0 {
                p.displace(Vec2::new(-dist_x, 0.0), 1.0);
            }

            let dist_x = p.pos.x + p.radius;
            if dist_x > screen_width() {
                p.displace(Vec2::new(screen_width() - dist_x, 0.0), 1.0);
            }
        }

        // build sorted lists
        let mut x_axis = AxisChain::new(10., self.particles.len());
        let mut y_axis = AxisChain::new(10., self.particles.len());

        for i in 0..self.particles.len() {
            let p = &self.particles[i];
            x_axis.set(i, p.pos.x);
            y_axis.set(i, p.pos.y);
        }

        x_axis.build();
        y_axis.build();

        // find intersections
        let mut intersections: Vec<(usize, usize)> = vec![];
        for i in 0..self.particles.len() {
            let x_index = x_axis.get(i).0;
            let y_index = y_axis.get(i).0;

            let x_chain = x_axis.find_chain(x_index);
            let y_chain = y_axis.find_chain(y_index);

            let mut set: HashSet<usize> = HashSet::new();
            {
                for v in x_chain.iter() {
                    if !set.contains(v) {
                        set.insert(*v);
                    }
                }
                for v in y_chain.iter() {
                    if !set.contains(v) {
                        set.insert(*v);
                    }
                }
            }

            // intersection
            for v in set.iter() {
                if x_chain.binary_search(v).is_ok() && y_chain.binary_search(v).is_ok() {
                    intersections.push((i, *v));
                }
            }
        }

        // store displace values
        let mut displacements: Vec<Vec2> = vec![Vec2::ZERO; self.particles.len()];

        // particle collisions
        for (i, j) in intersections.iter() {
            let p1 = &self.particles[*i];
            let p2 = &self.particles[*j];
            let delta = p1.pos - p2.pos;
            let dist = delta.length();
            let diff = p1.radius + p2.radius - dist;
            if diff > 0.0 {
                let normal = delta / dist;
                displacements[*i] += normal * diff * 0.5;
                displacements[*j] -= normal * diff * 0.5;
            }
        }
        
        // for i in 0..self.particles.len() {
        //     for j in 0..self.particles.len() {
        //         if i == j {
        //             continue;
        //         }

        //         let p1 = &self.particles[i];
        //         let p2 = &self.particles[j];
        //         let delta = p1.pos - p2.pos;
        //         let dist = delta.length();
        //         let diff = p1.radius + p2.radius - dist;
        //         if diff > 0.0 {
        //             let normal = delta / dist;
        //             displacements[i] += normal * diff * 0.5;
        //             displacements[j] -= normal * diff * 0.5;
        //         }
        //     }
        // }

        // apply displacements
        for i in 0..self.particles.len() {
            self.particles[i].displace(displacements[i], 0.5);
        }
    }

    async fn solve_input(&mut self) {
        if is_key_down(KeyCode::Space) {
            for _ in 0..100 {
                self.particles.push(Particle::new(
                    self.particles.len(),
                    Vec2::new(
                        rand::gen_range(0, screen_width() as u32) as f32,
                        rand::gen_range(0, screen_height() as u32) as f32,
                    ),
                    1.0,
                    10.0,
                ));
            }
        }
    }
}

pub async fn main() {
    let world = &mut World {
        particles: vec![],
        dt: 0.0,
    };

    loop {
        world.draw().await;
        world.solve_input().await;
        world.solve_physics();
    }
}
