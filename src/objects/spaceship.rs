use crate::objects::camera::Camera;
use crate::objects::interface::GameObject;
use crate::objects::move_data::MoveData;
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::{Context, GameResult};
use nalgebra::{self, Vector2};

pub struct Spaceship {
    move_data: MoveData,
    max_speed: f32,
    max_rotation: f32,
    fwd_speed: f32,
}

impl Spaceship {
    pub fn new(starting_pos: &Vector2<f32>) -> GameResult<Spaceship> {
        Ok(Spaceship {
            move_data: MoveData::new(starting_pos, 0.0),
            max_speed: 150.0,
            max_rotation: 1.5,
            fwd_speed: 0.0,
        })
    }

    pub fn forwards(&self) -> Vector2<f32> {
        Vector2::new(self.move_data.angle.cos(), self.move_data.angle.sin())
    }

    pub fn rotate(&mut self, speed: f32) {
        self.move_data.rot_speed = speed.signum() * speed.abs().min(1.0) * self.max_rotation;
    }

    pub fn move_by(&mut self, speed: f32) {
        self.fwd_speed = speed.signum() * speed.abs().min(1.0) * self.max_speed;
    }
}

impl GameObject for Spaceship {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.move_data.velocity = self.forwards() * self.fwd_speed;
        self.move_data.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, camera: &Camera) -> GameResult {
        let shape = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[[-25.0, -25.0], [50.0, 0.0], [-25.0, 25.0], [0.0, 0.0]],
            Color::from_rgb(0, 255, 255),
        )?;
        let draw_param = camera.to_view_coords(&self.move_data);
        graphics::draw(ctx, &shape, draw_param)
    }
}
