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
    // let mut state = MainState::new(font);
    
    // // let img = image::open("/home/enrico/Pictures/jose-raul-capablanca.jpg").unwrap();
    // let img = image::open("/home/enrico/Pictures/Nokia 6600/Foto(031).jpg").unwrap();

    // let (width, height) = img.dimensions();
    // println!("img {}x{}", width, height);

    // let grid_width = 800.0 / flow::SIZE;
    // let grid_height = 600.0 / flow::SIZE;

    // println!("grid {}x{}", grid_width, grid_height);

    // // let grid_max = grid_width.max(grid_height);

    // // let img_max = width.max(height);

    // let width_coeff = width as f32 / grid_width;
    // let height_coeff = height as f32 / grid_height;

    // let coeff = width_coeff.max(height_coeff);

    // let new_img = image::imageops::resize(&img, (width as f32 / coeff) as u32, (height as f32 / coeff) as u32, image::FilterType::Gaussian);
    
    // let (new_width, new_height) = new_img.dimensions();

    // println!("new img {}x{}", new_width, new_height);
    
    let args: Vec<String> = env::args().collect();

    let mut state = FlowState::new(font, args);

    event::run(ctx, &mut state)
}
