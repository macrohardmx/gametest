use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{timer, Context, GameResult};
use nalgebra::Vector2;

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
    fn update(&self, ctx: &mut Context, move_data: &mut MoveData) -> GameResult;
}

pub struct UserController {
    speed: f32,
}

impl UserController {
    pub fn new(speed: f32) -> UserController {
        UserController { speed }
    }
}

impl MoveController for UserController {
    fn update(&self, ctx: &mut Context, move_data: &mut MoveData) -> GameResult {
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

        if dir.norm() == 0.0 {
            move_data.velocity = dir;
        } else {
            move_data.velocity = dir / dir.norm() * self.speed;
        }
        Ok(())
    }
}
