use crate::objects::camera::Camera;
use crate::objects::interface::GameObject;
use crate::objects::move_data::{MoveController, MoveData, UserController};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::{Context, GameResult};
use nalgebra::{self, Vector2};
use std::rc::Rc;

pub struct Player {
    move_data: MoveData,
    move_controller: Rc<dyn MoveController>,
}

impl Player {
    pub fn new(starting_pos: &Vector2<f32>) -> GameResult<Player> {
        Ok(Player {
            move_data: MoveData::new(starting_pos, 0.0),
            move_controller: Rc::new(UserController::new(150.0)),
        })
    }
}

impl GameObject for Player {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.move_controller.update(ctx, &mut self.move_data)?;
        self.move_data.update(ctx)
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
