use crate::objects::background::Background;
use crate::objects::camera::Camera;
use crate::objects::interface::GameObject;
use crate::objects::player::Player;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
use nalgebra::{Point2, Vector2};

pub struct Level {
    player: Player,
    camera: Camera,
    background: Background,
}

impl Level {
    pub fn new(ctx: &mut Context) -> GameResult<Level> {
        let (w, h) = graphics::drawable_size(&ctx);

        Ok(Level {
            player: Player::new(&Vector2::new(w / 2.0, h / 2.0))?,
            camera: Camera::new(&Vector2::new(w / 2.0, h / 2.0), &Vector2::new(w, h)),
            background: Background::new(
                ctx,
                &Point2::new(h, h),
                &Point2::new(w / 2.0, h / 2.0),
                "/bg.png",
            )?,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(158, 45, 0));
        self.background.draw(ctx)?;
        self.player.draw(ctx, &self.camera)
    }
}
