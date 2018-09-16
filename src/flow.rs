use std;
use std::ops::IndexMut;
use std::time::Duration;
use std::time::Instant;
use std::path::Path;

use ggez::*;
use ggez::event::{EventHandler, Keycode, Mod, MouseState, MouseButton};
use ggez::graphics::{Color};
use ggez::timer::{get_delta, duration_to_f64};
use ggez::timer::{get_fps};
use image;

use fps::*;
use sync_timer::*;
use quad::*;

const SIZE : f32 = 10.0;
const MARGIN : f32 = 0.5;
const FLIP_VELOCITY : f64 = 2.0;
const FLIP_DELAY : u64 = 100;

lazy_static! {
    pub static ref UP_COLOR: Color = {
        Color::new(0.0, 0.0, 0.0, 1.0)
    };

    pub static ref DOWN_COLOR: Color = {
        Color::new(0.0, 0.5, 0.0, 1.0)
    };

    pub static ref BLACK: Color = {
        Color::new(0.0, 0.0, 0.0, 1.0)
    };
}

pub struct FlowState {
    font: graphics::Font,
    quads: Vec<Quad>,
    ctrl: bool,
    swapping_column: Option<usize>,
    timer: SyncTimer,
    last_fps_print: Instant,
    file_names: Vec<String>,
    file_index: usize,
    down_color: bool,
}

impl FlowState {

    pub fn new(font: graphics::Font, args: Vec<String>) -> FlowState {
        let folder_name = args.get(1);

        if folder_name.is_none() {
            println!("folder is mandatory", );
            panic!();
        }

        let directory = Path::new(folder_name.unwrap());

        let paths = directory.read_dir().unwrap();

        let mut file_names = Vec::new();

        for path in paths {
            if let Ok(entry) = path {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let file_name = String::from(entry.path().to_str().unwrap());
                        if file_name.to_uppercase().ends_with("PNG") ||
                            file_name.to_uppercase().ends_with("JPG") ||
                            file_name.to_uppercase().ends_with("JPEG") ||
                            file_name.to_uppercase().ends_with("BMP") {
                                file_names.push(file_name);
                            }
                    }
                }
            }
        }

        if file_names.is_empty() {
            println!("No image files found in {}", folder_name.unwrap());
            panic!();
        }

        let mut quads = Vec::new();
        let mut x = 0.0;
        let mut y = 0.0;

        while x < 800.0 {
            y = 0.0;
            while y < 600.0 {
                let quad = Quad::new(x + MARGIN, y + MARGIN, SIZE - 2.0 * MARGIN, SIZE - 2.0 * MARGIN, *BLACK, *BLACK, 0.0, 0.0);
                quads.push(quad);
                y += SIZE;
            }
            x += SIZE;
        }

        let mut timer = SyncTimer::new();
        timer.add(SyncEvent::new("swap_column", Duration::from_millis(FLIP_DELAY), true));

        FlowState{quads, font: font, ctrl: false, swapping_column: None, timer: timer, last_fps_print: Instant::now(),
            file_names: file_names, file_index: 0, down_color: true}
    }

    fn find_quad(&mut self, x: f32, y: f32) -> Option<&mut Quad> {
        for quad in self.quads.iter_mut() {
            if x >= quad.x && x <= quad.x + quad.width {
                if y >= quad.y && y  <= quad.y + quad.height {
                    return Some(quad);
                }
            }
        }
        None
    }

    fn swap_column_quads(&mut self, column: usize) {
        let x = column as f32 * SIZE + SIZE / 2.0;
        for quad in self.quads.iter_mut() {
            if x >= quad.x && x <= quad.x + quad.width {
                quad.flip_right(FLIP_VELOCITY);
                // println!("swapping {}", x);
            }
        }
    }

    fn load_image(&mut self) {
        let file_name = self.file_names.get(self.file_index).unwrap();
        println!("loading image {}", file_name);
        let image = load_and_resize_image(file_name);

        let img = if image.is_err() {
            println!("cannot load image {}: {}", file_name, image.unwrap_err());
            panic!();
        } else {
            image.unwrap()
        };

        let (img_width, img_height) = img.dimensions();

        for quad in self.quads.iter_mut() {
            let ix = (quad.x / SIZE).round() as u32;
            let iy = (quad.y / SIZE).round() as u32;

            let color = if ix >= img_width || iy >= img_height {
                *BLACK
            } else {
                pixel_to_color(img.get_pixel(ix, iy))
            };

            if self.down_color {
                quad.set_down_color(color);
            } else {
                quad.set_up_color(color);
            }
        }

        self.file_index += 1;

        if self.file_index >= self.file_names.len() {
            self.file_index = 0;
        }

    }

}

impl EventHandler for FlowState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let delta_duration = get_delta(_ctx);
        let delta = duration_to_f64(delta_duration);

        let mut i: usize = 0;
        while i < self.quads.len() {
            let quad = self.quads.index_mut(i);
            quad.update(delta);
            i += 1;
        }

        let fired = self.timer.fired().clone();

        for id in fired {
            if id == "swap_column" {
                if self.swapping_column.is_none() {
                    self.load_image();
                    self.swapping_column = Some(0);
                } else if (self.swapping_column.unwrap() as f32) < (800.0 / SIZE) {
                    {
                        let sw = self.swapping_column.unwrap();
                        self.swap_column_quads(sw);
                        // println!("swapping column {}", sw);
                    }
                    self.swapping_column = Some(self.swapping_column.unwrap() + 1);
                } else {
                    // println!("restart swap", );
                    self.swapping_column = None;
                    self.down_color = !self.down_color;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // graphics::clear(ctx);

        graphics::set_color(ctx, Color::new(0.0, 0.0, 0.0, 255.0))?;

        for quad in &self.quads {
            if !quad.is_updated() {
               continue; 
            }
            quad.draw_bk(ctx)?;
        }

        for quad in self.quads.iter_mut() {
            quad.draw(ctx)?;
        }

        // draw_fps(ctx, &self.font, graphics::Point2::new(10.0, 10.0), graphics::Color::from((255, 255, 255, 255)))?;

        // if Instant::now().duration_since(self.last_fps_print).as_secs() >= 1 {
        //     let fps = get_fps(ctx).round();
        //     println!("fps: {}", fps);
        //     self.last_fps_print = Instant::now();
        // }

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            /*Keycode::R => {
                let mut i: usize = 0;
                while i < self.quads.len() {
                    let quad = self.quads.index_mut(i);
                    quad.angle = 0.0;
                    quad.va = 0.0;
                    i += 1;
                }     
            },*/
            Keycode::RCtrl | Keycode::LCtrl => self.ctrl = true,
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if keycode == Keycode::RCtrl || keycode == Keycode::LCtrl {
            self.ctrl = false;
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        if self.ctrl {
            match self.find_quad(x as f32, y as f32) {
                Some(quad) => 
                    if quad.faced_up() {
                        quad.flip_right(FLIP_VELOCITY);
                    } else {
                        quad.flip_left(FLIP_VELOCITY);
                    },
                _ => ()
            };
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: i32,
        y: i32,
    ) {
        match self.find_quad(x as f32, y as f32) {
            Some(quad) => 
                if quad.faced_up() {
                    quad.flip_right(FLIP_VELOCITY);
                } else {
                    quad.flip_left(FLIP_VELOCITY);
                },
            _ => ()
        };
    }

}

fn pixel_to_color(pixel: &image::Rgba<u8>) -> Color {
    Color::new(pixel.data[0] as f32 / 255.0, pixel.data[1] as f32 / 255.0, pixel.data[2] as f32 / 255.0, 
                        pixel.data[3] as f32 / 255.0)
}

fn load_and_resize_image(file: &str) -> image::ImageResult<image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>> {
    let maybe_img = image::open(&file)?;
    // println!("file opened", );

    let img = maybe_img.to_rgba();

    let (width, height) = img.dimensions();
    // println!("img {}x{}", width, height);

    let grid_width = 800.0 / SIZE;
    let grid_height = 600.0 / SIZE;

    // println!("grid {}x{}", grid_width, grid_height);

    let width_coeff = width as f32 / grid_width;
    let height_coeff = height as f32 / grid_height;

    let coeff = width_coeff.max(height_coeff);

    let new_img = image::imageops::resize(&img, 
        (width as f32 / coeff) as u32, 
        (height as f32 / coeff) as u32, 
        image::FilterType::Gaussian);
    
    // let (new_width, new_height) = new_img.dimensions();
    // println!("new img {}x{}", new_width, new_height);

    Ok(new_img)

}
