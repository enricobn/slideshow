extern crate gfx;
extern crate ggez;
extern crate image;
extern crate lazy_static;
extern crate rand;
extern crate separator;

use std::env;
use std::ops::Deref;
use std::path;

use ggez::conf::FullscreenType;
use ggez::winit::platform::run_return::EventLoopExtRunReturn;
use ggez::*;

use crate::slideshow::*;

mod ggez_utils;
mod slideshow;
mod sync_timer;
mod transitions;
mod utils;
mod velocity;

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

    let window_mode = conf::WindowMode::default()
        .dimensions(1920.0, 1080.0)
        .fullscreen_type(FullscreenType::Desktop);

    let cb = ContextBuilder::new("slideshow", "enricobn")
        .window_setup(conf::WindowSetup::default().title("Slideshow").vsync(true))
        .window_mode(window_mode)
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = cb.build()?;

    event::run(ctx, events_loop, state)
}
