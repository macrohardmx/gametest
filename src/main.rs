mod objects;

use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::{self, quit, EventHandler, KeyCode, KeyMods};
use ggez::graphics::Color;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use nalgebra::{Point2, Vector2};

use std::cell::RefCell;
use std::env;
use std::path;
use std::rc::Rc;

use objects::background::Background;
use objects::camera::Camera;
use objects::interface::GameObject;
use objects::player::Player;

fn main() {
    let resource_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_dir) => {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        }
        Err(_) => path::PathBuf::from("./resources"),
    };

    let (mut ctx, mut event_loop) =
        ContextBuilder::new("my_game", "Ricardo Delfin <me@rdelfin.com>")
            .add_resource_path(resource_dir)
            .window_setup(WindowSetup::default().title("Game Test (rdelfin)"))
            .window_mode(WindowMode::default().fullscreen_type(FullscreenType::True))
            .build()
            .expect("Could not create ggez context!");

    // Create an instance of the event handler
    // Usually, you want to provide it with the
    // context object to use when setting your game up
    let mut game = MyGame::new(&mut ctx).unwrap();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    camera: Camera,
    game_objects: Vec<Rc<RefCell<dyn GameObject>>>,
    cam_speed: Vector2<f32>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let (w, h) = graphics::drawable_size(&ctx);
        let screen_res = Point2::new(w, h);

        let player = Rc::new(RefCell::new(Player::new(&Vector2::new(100.0, 100.0))?));
        let background = Rc::new(RefCell::new(Background::new(
            ctx,
            &screen_res,
            "/rancho_relaxo.jpg",
        )?));
        Ok(MyGame {
            game_objects: vec![background.clone(), player.clone()],
            camera: Camera::new(
                &Vector2::new(screen_res.x / 2.0, screen_res.y / 2.0),
                &Vector2::new(screen_res.x, screen_res.y),
            ),
            cam_speed: Vector2::new(0.0, 0.0),
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.camera.move_by(&self.cam_speed);

        for obj_cell in &mut self.game_objects {
            obj_cell.borrow_mut().update(ctx, &self.camera)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(100, 149, 237));
        for obj_cell in &mut self.game_objects {
            obj_cell.borrow_mut().draw(ctx, &self.camera)?;
        }
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            // Camera Movement
            KeyCode::Up => {
                self.cam_speed.y = -3.0;
            }
            KeyCode::Down => {
                self.cam_speed.y = 3.0;
            }
            KeyCode::Left => {
                self.cam_speed.x = -3.0;
            }
            KeyCode::Right => {
                self.cam_speed.x = 3.0;
            }
            // Quit
            KeyCode::Escape => {
                quit(ctx);
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            // Camera movement
            KeyCode::Up => {
                self.cam_speed.y = 0.0;
            }
            KeyCode::Down => {
                self.cam_speed.y = 0.0;
            }
            KeyCode::Left => {
                self.cam_speed.x = 0.0;
            }
            KeyCode::Right => {
                self.cam_speed.x = 0.0;
            }

            _ => {}
        }
    }
}
