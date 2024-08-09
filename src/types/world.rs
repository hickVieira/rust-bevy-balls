use macroquad::prelude::*;
use crate::types::ball::*;
use crate::consts::*;

pub struct World {
    pub balls: Vec<Ball>,
    pub frame_time: f32,
    pub physics_time: f32,
    pub system_energy: f32,
}

impl World {
    pub async fn draw(&mut self) {
        let timer = std::time::Instant::now();
        clear_background(WHITE);
        for p in self.balls.iter() {
            p.draw();
        }
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("Press SPACE to add balls", 20.0, 40.0, 30.0, DARKGRAY);
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
            &format!("Balls: {}", self.balls.len()),
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

    pub fn solve_physics(&mut self) {
        let timer = std::time::Instant::now();

        // get system energy
        self.system_energy = {
            let mut sum = 0.0;
            for p in self.balls.iter() {
                sum += p.vel.length();
            }
            sum / (self.balls.len() as f32)
        };

        // add gravity
        // for p in self.particles.iter_mut() {
        //     p.apply_acceleration(GRAVITY * PHY_DT);
        // }

        // integrate
        for p in self.balls.iter_mut() {
            for _ in 0..PHY_ITERATIONS {
                p.integrate(PHY_SDT);
            }
        }

        // wall collisions
        for p in self.balls.iter_mut() {
            let dist_y = p.pos.y - p.radius;
            if dist_y < 0.0 {
                p.displace(Vec2::new(0.0, -dist_y));
                p.vel = Vec2::new(p.vel.x, -p.vel.y);
            }

            let dist_x = p.pos.x - p.radius;
            if dist_x < 0.0 {
                p.displace(Vec2::new(-dist_x, 0.0));
                p.vel = Vec2::new(-p.vel.x, p.vel.y);
            }

            let dist_x = p.pos.x + p.radius;
            if dist_x > screen_width() {
                p.displace(Vec2::new(screen_width() - dist_x, 0.0));
                p.vel = Vec2::new(-p.vel.x, p.vel.y);
            }
        }

        // particle collisions
        let mut displacements: Vec<Vec2> = vec![Vec2::ZERO; self.balls.len()];
        let mut velocities: Vec<Vec2> = vec![Vec2::ZERO; self.balls.len()];

        // particle collisions O(n^2)
        for i in 0..self.balls.len() {
            for j in 0..self.balls.len() {
                if i == j {
                    continue;
                }

                let p1 = &self.balls[i];
                let p2 = &self.balls[j];
                let delta = p1.pos - p2.pos;
                let dist = delta.length();
                let diff = p1.radius + p2.radius - dist;

                if diff < 0.0 {
                    continue;
                }

                // if dist == 0.0 || dist > p1.radius + p2.radius {
                //     continue;
                // }

                let normal = delta / dist;

                let v1 = p1.vel.dot(normal);
                let v2 = p2.vel.dot(normal);
                let m1 = p1.mass;
                let m2 = p2.mass;

                let m1v1m2v2 = m1 * v1 + m2 * v2;
                let m1m2 = m1 + m2;

                let m2v1v2e = m2 * (v1 - v2) * ELASTICITY;
                let m1v2v1e = m1 * (v2 - v1) * ELASTICITY;

                let v1_new = (m1v1m2v2 - m2v1v2e) / m1m2;
                let v2_new = (m1v1m2v2 - m1v2v1e) / m1m2;

                displacements[i] += normal * diff * 0.5;
                displacements[j] -= normal * diff * 0.5;

                velocities[i] += normal * (v1_new - v1) * 0.5;
                velocities[j] += normal * (v2_new - v2) * 0.5;
            }
        }

        // apply displacements
        for i in 0..self.balls.len() {
            self.balls[i].displace(displacements[i]);
            self.balls[i].apply_velocity(velocities[i]);
        }
        self.physics_time = timer.elapsed().as_secs_f32();
    }

    pub async fn solve_input(&mut self) {
        if is_key_down(KeyCode::Space) {
            let radius = rand::gen_range(10.0, 50.0);
            for _ in 0..10 {
                self.balls.push(Ball::new(
                    Vec2::new(
                        rand::gen_range(0, screen_width() as u32) as f32,
                        rand::gen_range(0, screen_height() as u32) as f32,
                    ),
                    PI * radius * radius,
                    radius,
                    Vec2::new(
                        rand::gen_range(-500.0, 500.0),
                        rand::gen_range(-500.0, 500.0),
                    ),
                ));
            }
        }

        if is_key_down(KeyCode::R) {
            self.balls.clear();
        }
    }
}
