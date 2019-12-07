use ggez::graphics::{self, BlendMode, Color, DrawMode, DrawParam, Drawable, Mesh, Rect};
use ggez::{Context, GameResult};
use nalgebra::{self, Point2, Vector2};

pub struct Bullet {
    position: Point2<f32>,
    direction: Vector2<f32>,
    blend_mode: Option<BlendMode>,
    speed: f32,
}

impl Bullet {
    pub fn new(position: Point2<f32>, direction: Vector2<f32>) -> Bullet {
        let direction = direction / direction.norm();
        Bullet {
            position,
            direction,
            blend_mode: None,
            speed: 10.0,
        }
    }

    pub fn update(&mut self) -> GameResult {
        self.position += self.direction * self.speed;
        Ok(())
    }

    pub fn get_direction(&self) -> &Vector2<f32> {
        &self.direction
    }
}

impl Drawable for Bullet {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let shape = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [0.0, 0.0],
            5.0,
            0.5,
            Color::from_rgb(255, 0, 0),
        )?;
        graphics::draw(ctx, &shape, param.dest(Point2::from(self.position)))
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(Rect::new(-5.0, -5.0, 5.0, 5.0))
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.blend_mode = mode;
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.blend_mode
    }
}
