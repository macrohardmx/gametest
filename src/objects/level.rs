use crate::objects::camera::Camera;
use crate::objects::interface::GameObject;
use ggez::{Context, GameResult};
use nalgebra::Vector2;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LevelWrapper<T> {
    objects: Vec<Rc<RefCell<dyn GameObject>>>,
    camera: Camera,
    level: T,
}

impl<T: Level> LevelWrapper<T> {
    pub fn new(screen_dims: Vector2<f32>, level: T) -> Level {
        Level {
            objects: vec![],
            camera: Camera::new(
                Vector2::new(-screen_dims.x / 2.0, -screen_dims.y / 2.0),
                screen_dims,
            ),
            level,
        }
    }

    pub fn load(&mut self) -> GameResult {}

    pub fn update(&mut self) -> GameResult {
        for obj_cell in self.objects {
            obj_cell.borrow_mut().update(ctx)?;
        }
    }

    pub fn draw(&mut self) -> GameResult {
        for obj_cell in self.objects {
            obj_cell.borrow_mut().draw(ctx, &self.camera)?;
        }
    }
}

pub trait Level {
    fn load(&mut self, &mut ctx);
}
