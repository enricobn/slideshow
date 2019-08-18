extern crate ggez;
extern crate rand;
extern crate image;

#[macro_use]
extern crate lazy_static;

mod flow;
mod sync_timer;
mod quad;
mod globals;
mod ggez_utils;
mod grid;
mod transitions;
mod slideshow;
mod velocity;

use std::env;
use std::path;
use flow::*;
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

        let cb = ContextBuilder::new("screensaver", "ggez")
            .window_setup(conf::WindowSetup::default())
            .window_mode(window_mode);

        let (ctx, events_loop) = &mut cb.build()?;

        graphics::drawable_size(ctx)
    };

    let window_mode = conf::WindowMode::default().fullscreen_type(FullscreenType::True)
        .dimensions(width, height);

    let cb = ContextBuilder::new("screensaver", "ggez")
        .window_setup(
            conf::WindowSetup::default()
                .title("Screensaver")
                .vsync(true)
        )
        .window_mode(window_mode)
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("Drawable size {:?}", graphics::drawable_size(ctx));
    println!("Screen coordinates {:?}", graphics::screen_coordinates(ctx));

    /*
    let mut c = conf::Conf::new();
    c.window_setup = c.window_setup.title("Slideshow");
    // c.window_mode.fullscreen_type = FullscreenType::Desktop;
    c.window_mode.vsync = true;
    //c.window_mode.width = 1680;
    //c.window_mode.height = 1050;

    println!("screen: {}x{}", c.window_mode.width, c.window_mode.height);

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c)?;

    let modes = ggez::graphics::get_fullscreen_modes(ctx, 0)?;


    if modes.len() == 0 {
        panic!("No full screen modes available!"); // TODO  switch to windowed version
    }

    let width = modes[0].0; 
    let height = modes[0].1;

    let window_mode = WindowMode {
        width: width,
        height: height,
        borderless: false,
        fullscreen_type: FullscreenType::True,
        vsync: true,
        min_width: width,
        max_width: height,
        min_height: width,
        max_height: height,
    };

    graphics::set_mode(ctx, window_mode)?;
    */

    //graphics::set_background_color(ctx, graphics::Color::from((0, 0, 0, 255)));


    //c.window_mode.fullscreen_type = FullscreenType::Desktop;

    //for (width, height) in modes {
    //    println!("{}x{}", width, height);
    //}

    let args: Vec<String> = env::args().collect();

    let mut state = SlideShow::new(args); //FlowState::new(font, args);

    event::run(ctx, events_loop, &mut state)
}
