use crate::objects::camera::Camera;
use crate::objects::interface::GameObject;
use crate::objects::move_data::MoveData;

use ggez::graphics::{DrawParam, Image};
use ggez::{graphics, Context, GameResult};
use nalgebra::{Point2, Vector2};

pub struct Background {
    image: Image,
    move_data: MoveData,
}

impl Background {
    pub fn new(ctx: &mut Context, screen_res: &Point2<f32>, path: &str) -> GameResult<Background> {
        let image = Image::new(ctx, path)?;
        let dims: Vector2<f32> = screen_res - Point2::new(0.0, 0.0);
        Ok(Background {
            image,
            move_data: MoveData::new(&(-dims), 0.0),
        })
    }
}

impl GameObject for Background {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, camera: &Camera) -> GameResult {
        graphics::draw(ctx, &self.image, camera.to_view_coords(&self.move_data))
    }
}
