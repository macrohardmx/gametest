use crate::objects::camera::Camera;
use ggez::{Context, GameResult};

pub trait GameObject {
    fn update(&mut self, ctx: &mut Context, camera: &Camera) -> GameResult;
    fn draw(&mut self, ctx: &mut Context, camera: &Camera) -> GameResult;
}
