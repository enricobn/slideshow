extern crate ggez;
extern crate rand;
extern crate image;
extern crate gfx;

#[macro_use]
extern crate lazy_static;

mod sync_timer;
mod globals;
mod ggez_utils;
mod transitions;
mod slideshow;
mod velocity;

use std::env;
use std::path;
use slideshow::*;

use ggez::*;
use ggez::conf::{FullscreenType, WindowMode};
use ggez::graphics::{self};

fn main() -> GameResult<()> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("../resources")
    };

    // TODO it's a bit weird, I create a context only to know the actual screen size,
    // then I use it to build the real context!
    let (width, height) = {
        let window_mode = conf::WindowMode::default().fullscreen_type(FullscreenType::True);

        let cb = ContextBuilder::new("slideshow", "ggez")
            .window_setup(conf::WindowSetup::default())
            .window_mode(window_mode);

        let (ctx, events_loop) = &mut cb.build()?;

        graphics::drawable_size(ctx)
    };

    let window_mode = conf::WindowMode::default().fullscreen_type(FullscreenType::True)
        .dimensions(width, height);

    let cb = ContextBuilder::new("slideshow", "enricobn")
        .window_setup(
            conf::WindowSetup::default()
                .title("Slideshow")
                .vsync(true)
        )
        .window_mode(window_mode)
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("Drawable size {:?}", graphics::drawable_size(ctx));
    println!("Screen coordinates {:?}", graphics::screen_coordinates(ctx));

    let args: Vec<String> = env::args().collect();

    let mut state = SlideShow::new(args);

    event::run(ctx, events_loop, &mut state)
}
