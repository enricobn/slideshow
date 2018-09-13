extern crate ggez;
extern crate rand;

mod fight;
mod flow;

use std::env;
use std::path;
use fight::*;
use flow::*;

use ggez::*;
use ggez::conf::FullscreenType;

fn main() -> GameResult<()> {
    let mut c = conf::Conf::new();
    c.window_setup = c.window_setup.title("Quad fight");
    // c.window_mode.fullscreen_type = FullscreenType::Desktop;
    // c.window_mode.width = 60;
    // c.window_mode.height = 60;
    c.window_mode.vsync = false;

    println!("screen: {}x{}", c.window_mode.width, c.window_mode.height);

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c)?;

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    for (width, height) in ggez::graphics::get_fullscreen_modes(ctx, 0)? {
        println!("{}x{}", width, height);
    }

    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 16)?;
    // let mut state = MainState::new(font);
    let mut state = FlowState::new(font);

    event::run(ctx, &mut state)
}
