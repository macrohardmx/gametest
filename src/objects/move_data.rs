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

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta = timer::delta(ctx);
        let time_secs = (delta.as_secs() as f32) + (delta.subsec_millis() as f32 / 1000.0);
        self.position += self.velocity * time_secs;
        self.angle += self.rot_speed * time_secs;
        Ok(())
    }
}
