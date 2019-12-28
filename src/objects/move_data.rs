use crate::objects::camera::Camera;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::input::mouse;
use ggez::{timer, Context, GameResult};
use nalgebra::{Vector2, Point2};

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

    pub fn forwards(&self) -> Vector2<f32> {
        Vector2::new(self.angle.cos(), self.angle.sin())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = timer::delta(ctx);
        let time_secs = (delta.as_secs() as f32) + (delta.subsec_millis() as f32 / 1000.0);
        self.position += self.velocity * time_secs;
        self.angle += self.rot_speed * time_secs;
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
            dir.y = 1.0;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            dir.y = -1.0;
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

        // Rotate player with mouse
        let mouse_pos = camera.point_s2w(Point2::from(mouse::position(ctx)));
        move_data.angle = (mouse_pos.y - move_data.position.y).atan2(mouse_pos.x - move_data.position.x);

        Ok(())
    }
}