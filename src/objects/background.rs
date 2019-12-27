use ggez::graphics::{DrawParam, Image};
use ggez::{graphics, Context, GameResult};
use nalgebra::{Point2, Vector2};

pub struct Background {
    image: Image,
    target_res: Point2<f32>,
    center: Point2<f32>,
}

impl Background {
    pub fn new(
        ctx: &mut Context,
        target_res: &Point2<f32>,
        center: &Point2<f32>,
        path: &str,
    ) -> GameResult<Background> {
        let image = Image::new(ctx, path)?;
        Ok(Background {
            image,
            target_res: target_res.clone(),
            center: center.clone(),
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let scale = Vector2::new(
            self.target_res.x / self.image.width() as f32,
            self.target_res.y / self.image.height() as f32,
        );
        let dest_pos = self.center - (self.target_res / 2.0);
        graphics::draw(
            ctx,
            &self.image,
            DrawParam::new().dest(Point2::from(dest_pos)).scale(scale),
        )
    }
}
