use crate::objects::move_data::MoveData;
use ggez::graphics::DrawParam;
use nalgebra::{self, Point2, Vector2};

pub struct Camera {
    center: Vector2<f32>,
    play_area_dims: Vector2<f32>,
}

impl Camera {
    pub fn new<T: Into<Vector2<f32>>>(center: T, play_area_dims: T) -> Camera {
        Camera {
            center: center.into(),
            play_area_dims: play_area_dims.into(),
        }
    }

    pub fn move_by<T: Into<Vector2<f32>>>(&mut self, diff: T) {
        self.center += diff.into();
    }

    pub fn to_proj_coords(&self, move_data: &MoveData) -> DrawParam {
        // Coordinate system goes:
        // -1 on left, +1 on right, +1 top and -1 bottom

        let view_scale = Vector2::new(self.play_area_dims.x / 2.0, -self.play_area_dims.y / 2.0);
        let dest = Point2::from(move_data.position.component_mul(&view_scale) + self.center);

        DrawParam::new()
            .scale(view_scale)
            .dest(dest)
            .rotation(move_data.angle)
    }

    // Converts a point from screen coordinates to world coordinates
    pub fn point_s2w(&self, p: Point2<f32>) -> Point2<f32> {
        p + self.center - self.screen_dims / 2.0
    }
}
