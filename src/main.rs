mod objects;

use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::Color;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use std::env;
use std::path;

use objects::level::Level;

fn main() -> GameResult {
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
            .build()?;

    // Create an instance of the event handler
    // Usually, you want to provide it with the
    // context object to use when setting your game up
    let mut game = MyGame::new(&mut ctx)?;

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }

    Ok(())
}

struct MyGame {
    level: Level,
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<MyGame> {
        Ok(MyGame {
            level: Level::new(ctx)?,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.level.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(100, 149, 237));
        self.level.draw(ctx)?;
        graphics::present(ctx)
    }
}
