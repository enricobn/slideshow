extern crate ggez;
extern crate rand;
extern crate image;
#[macro_use]
extern crate lazy_static;

mod fight;
mod flow;
mod fps;
mod sync_timer;
mod quad;
mod globals;

use std::env;
use std::path;
use fight::*;
use flow::*;

use ggez::*;
use ggez::conf::FullscreenType;

use image::*;

fn main() -> GameResult<()> {
    let mut c = conf::Conf::new();
    c.window_setup = c.window_setup.title("Quad fight");
    // c.window_mode.fullscreen_type = FullscreenType::Desktop;
    // c.window_mode.width = 60;
    // c.window_mode.height = 60;
    c.window_mode.vsync = false;

    println!("screen: {}x{}", c.window_mode.width, c.window_mode.height);

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c)?;
    graphics::set_background_color(ctx, graphics::Color::from((0, 0, 0, 255)));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    for (width, height) in ggez::graphics::get_fullscreen_modes(ctx, 0)? {
        println!("{}x{}", width, height);
    }

    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 16)?;
     
    let args: Vec<String> = env::args().collect();

    let mut state = FlowState::new(font, args);

    event::run(ctx, &mut state)
}
