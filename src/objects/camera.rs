use crate::objects::move_data::MoveData;
use ggez::graphics::DrawParam;
use nalgebra::{self, Point2, Vector2};

pub struct Camera {
    center: Vector2<f32>,
    screen_dims: Vector2<f32>,
}

impl Camera {
    pub fn new(center: &Vector2<f32>, screen_dims: &Vector2<f32>) -> Camera {
        Camera {
            center: center.clone(),
            screen_dims: screen_dims.clone(),
        }
    }

    pub fn move_by(&mut self, diff: &Vector2<f32>) {
        self.center += diff;
    }

    pub fn to_view_coords(&self, move_data: &MoveData) -> DrawParam {
        let corner = self.center - self.screen_dims / 2.0;
        let final_pos = move_data.position - corner;
        DrawParam::new()
            .dest(Point2::new(final_pos.x, final_pos.y))
            .rotation(move_data.angle)
    }

    // Converts a point from screen coordinates to world coordinates
    pub fn point_s2w(&self, p: Point2<f32>) -> Point2<f32> {
        p - self.center + self.screen_dims / 2.0;
    }
}
