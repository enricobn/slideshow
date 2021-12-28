extern crate gfx;
extern crate ggez;
extern crate image;
extern crate lazy_static;
extern crate rand;
extern crate separator;

use std::env;
use std::path;

use ggez::*;
use ggez::conf::FullscreenType;
use ggez::graphics::{self};

use crate::slideshow::*;

mod sync_timer;
mod ggez_utils;
mod transitions;
mod slideshow;
mod velocity;
mod utils;

fn main() -> GameResult<()> {
    let args: Vec<String> = env::args().collect();

    let state = SlideShow::new(args);

    build_context_and_run(state)
}

fn build_context_and_run(state: SlideShow) -> Result<(), GameError> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("../resources")
    };

    let window_mode = conf::WindowMode::default().fullscreen_type(FullscreenType::True);

    /*let window_mode = conf::WindowMode::default().fullscreen_type(FullscreenType::Windowed)
        .dimensions(300.0, 300.0);
     */

    let cb = ContextBuilder::new("slideshow", "enricobn")
        .window_setup(
            conf::WindowSetup::default()
                .title("Slideshow")
                .vsync(false)
        )
        .window_mode(window_mode)
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = cb.build()?;

    println!("Drawable size {:?}", graphics::drawable_size(&ctx));
    println!("Screen coordinates {:?}", graphics::screen_coordinates(&ctx));

    event::run(ctx, events_loop, state)
}
