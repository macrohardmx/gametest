use crate::objects::background::Background;
use crate::objects::camera::Camera;
use crate::objects::interface::GameObject;
use crate::objects::player::Player;
use crate::objects::wall::WallSegment;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
use nalgebra::{Point2, Vector2};

pub struct Level {
    player: Player,
    camera: Camera,
    background: Background,
    wall: WallSegment,
}

impl Level {
    pub fn new(ctx: &mut Context) -> GameResult<Level> {
        let (w, h) = graphics::drawable_size(&ctx);

        Ok(Level {
            player: Player::new(Vector2::new(0.0, 0.0))?,
            camera: Camera::new(Vector2::new(w / 2.0, h / 2.0), Vector2::new(h, h)),
            background: Background::new(
                ctx,
                &Point2::new(h, h),
                &Point2::new(w / 2.0, h / 2.0),
                "/bg.png",
            )?,
            wall: WallSegment::new(
                ctx,
                Vector2::new(0.3, 0.3),
                Vector2::new(0.1, 0.1),
                "/wall.png",
            )?,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.wall.update(ctx, &self.camera)?;
        self.player.update(ctx, &self.camera)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(158, 45, 0));
        self.background.draw(ctx)?;
        self.wall.draw(ctx, &self.camera)?;
        self.player.draw(ctx, &self.camera)
    }
}
