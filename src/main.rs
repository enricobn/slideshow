extern crate ggez;
extern crate rand;
extern crate image;
#[macro_use]
extern crate lazy_static;

mod flow;
mod fps;
mod sync_timer;
mod quad;
mod globals;
mod grid;
mod transition;
mod slideshow;
mod pixels;

use std::env;
use std::path;
use flow::*;
use slideshow::*;

use ggez::*;
use ggez::conf::FullscreenType;

fn main() -> GameResult<()> {
    let mut c = conf::Conf::new();
    c.window_setup = c.window_setup.title("Screensaver");
    // c.window_mode.fullscreen_type = FullscreenType::Desktop;
    // c.window_mode.width = 60;
    // c.window_mode.height = 60;
    c.window_mode.vsync = true;

    println!("screen: {}x{}", c.window_mode.width, c.window_mode.height);

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c)?;
    graphics::set_background_color(ctx, graphics::Color::from((0, 0, 0, 255)));

    for (width, height) in ggez::graphics::get_fullscreen_modes(ctx, 0)? {
        println!("{}x{}", width, height);
    }
    
    let args: Vec<String> = env::args().collect();

    let mut state = SlideShow::new(args); //FlowState::new(font, args);

    event::run(ctx, &mut state)
}
