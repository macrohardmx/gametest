use crate::objects::camera::Camera;
use crate::objects::interface::GameObject;
use crate::objects::move_data::MoveData;
use ggez::graphics::{self, Image};
use ggez::{Context, GameResult};
use nalgebra::Vector2;

// Map works in a grid pattern
pub struct WallSegment {
    move_data: MoveData,
    image: Image,
    size: Vector2<f32>,
}

impl WallSegment {
    pub fn new<T: Into<Vector2<f32>>>(
        ctx: &mut Context,
        pos: T,
        size: T,
        path: &str,
    ) -> GameResult<WallSegment> {
        Ok(WallSegment {
            move_data: MoveData::new(pos, 0.0),
            image: Image::new(ctx, path)?,
            size: size.into(),
        })
    }
}

impl GameObject for WallSegment {
    fn update(&mut self, ctx: &mut Context, camera: &Camera) -> GameResult {
        self.move_data.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context, camera: &Camera) -> GameResult {
        let draw_param = camera.to_proj_coords(&self.move_data);
        let img_size = Vector2::new(self.image.width() as f32, self.image.height() as f32);
        let draw_param = camera.center_local_coords(draw_param, img_size, self.size);
        graphics::draw(ctx, &self.image, draw_param)
    }
}
