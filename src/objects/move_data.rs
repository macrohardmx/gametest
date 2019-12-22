use crate::objects::camera::Camera;
use std::f64::consts;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::input::mouse;
use ggez::{timer, Context, GameResult};
use nalgebra::Vector2;
// use nalgebra::base::Matrix;

pub struct MoveData {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub angle: f32,
    pub rot_speed: f32,
}

impl MoveData {
    pub fn new(position: &Vector2<f32>, angle: f32) -> MoveData {
        MoveData {
            position: position.clone(),
            velocity: Vector2::new(0.0, 0.0),
            angle,
            rot_speed: 0.0,
        }
    }

    // Is this even needed?
    // pub fn forwards(&self) -> Vector2<f32> {
        // Vector2::new(self.angle.cos(), self.angle.sin())
    // }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = timer::delta(ctx);
        let time_secs = (delta.as_secs() as f32) + (delta.subsec_millis() as f32 / 1000.0);
        self.position += self.velocity * time_secs;
        self.angle += self.rot_speed * time_secs; // rot_speed = f32, time_secs = f32
        Ok(())
    }
}

pub trait MoveController {
    fn update(&self, ctx: &mut Context, camera: &Camera, move_data: &mut MoveData) -> GameResult;
}

pub struct UserController {
    speed: f32,
    // Inertia goes from 0.0 to 1.0, where 0.0 means zero resistence (immediately goes full speed)
    // and 1.0 means full resistence (the object won't move)
    inertia: f32,
}

impl UserController {
    pub fn new(speed: f32, inertia: f32) -> UserController {
        UserController {
            speed,
            inertia: inertia.max(0.0).min(1.0),
        }
    }
}

impl MoveController for UserController {
    fn update(&self, ctx: &mut Context, camera: &Camera, move_data: &mut MoveData) -> GameResult {
        
        // Keyboard bindings to movement
        let mut dir = Vector2::new(0.0, 0.0);
        if keyboard::is_key_pressed(ctx, KeyCode::W) && !keyboard::is_key_pressed(ctx, KeyCode::S) {
            dir.y = -1.0;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            dir.y = 1.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) && !keyboard::is_key_pressed(ctx, KeyCode::A) {
            dir.x = 1.0;
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            dir.x = -1.0;
        }

        // Speed as f(inertia)
        let target_velocity = if dir.norm() == 0.0 {
            dir
        } else {
            dir / dir.norm() * self.speed
        };

        let diff_velocity = target_velocity - move_data.velocity;
        move_data.velocity += diff_velocity * (1.0 - self.inertia);

        /* Mouse input to angle (aka. What the fuck am I doing???)
        let m = mouse::position(ctx); // Type Point2
        let mouse_pos = Vector2::new(m.x, m.y); // Transformed to Vector2 to be read by Matrix
        let current_angle = Matrix::angle(&mouse_pos, &move_data.position); */

        let m = mouse::position(ctx);
        move_data.angle = ((m.y - move_data.position.y).atan2(move_data.position.x - m.x)) * (consts::PI / 2.0) as f32;
        /* let half_pi = (consts::PI / 2.0) as f32;
        move_data.angle = angle * half_pi;
        println!("{:?} : {:?}", m, half_pi); */
        // m gives x and y in pixels. Need to 1. Find mouse pos with respect to move_data.position in 0,0
        // 2. Find angle between mouse pos and player position with player pos as point of reference. (In radians)
        // 3. Change move_data.rotation value to this angle.

        Ok(())
    }
}