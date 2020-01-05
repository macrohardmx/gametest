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
        let view_scale = Vector2::new(self.play_area_dims.x / 2.0, -self.play_area_dims.y / 2.0);
        let rel_center = p - Point2::from(self.center);
        Point2::from(rel_center.component_div(&view_scale))
    }

    // Scale will default to level-based (-1 to 1) coordinates so we need to scale down
    pub fn center_local_coords(
        &self,
        draw_param: DrawParam,
        obj_size: Vector2<f32>,
        final_size: Vector2<f32>,
    ) -> DrawParam {
        let norm_scale = final_size.component_div(&obj_size);
        let center_offset = Vector2::from(draw_param.scale).component_mul(&(final_size / 2.0));
        let old_dest = Vector2::new(draw_param.dest.x, draw_param.dest.y);
        draw_param
            .scale(Vector2::from(draw_param.scale).component_mul(&norm_scale))
            .dest(Point2::from(old_dest - center_offset))
    }
}
