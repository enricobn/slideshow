use std;
use std::ops::Sub;
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
use grid::*;

const QUAD_SIZE : f32 = 10.0;
const QUAD_MARGIN : f32 = 0.0;
const FLIP_VELOCITY : f64 = 2.0;
const FLIP_DELAY : u64 = 100;
const LOAD_IMAGE_DELAY : u64 = 5_000; // millis
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

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
    grid: Grid,
    ctrl: bool,
    swapping_column: Option<usize>,
    timer: SyncTimer,
    last_fps_print: Instant,
    file_names: Vec<String>,
    file_index: usize,
    quad_side: QuadSide,
    last_ended_swap: Instant,
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

        let grid = Grid::new(WIDTH, HEIGHT, QUAD_SIZE, QUAD_MARGIN, *BLACK);

        let mut timer = SyncTimer::new();
        timer.add(SyncEvent::new("swap_column", Duration::from_millis(FLIP_DELAY), true));

        FlowState{grid: grid, font: font, ctrl: false, swapping_column: None, timer: timer, last_fps_print: Instant::now(),
            file_names: file_names, file_index: 0, quad_side: QuadSide::Down, 
            last_ended_swap: Instant::now().sub(Duration::from_millis(LOAD_IMAGE_DELAY))}
    }
    
    fn load_image(&mut self) {
        let file_name = self.file_names.get(self.file_index).unwrap();
        println!("loading image {}", file_name);
        let maybe_image = self.grid.load_image(file_name, &self.quad_side);

        if maybe_image.is_err() {
            println!("cannot load image {}: {}", file_name, maybe_image.unwrap_err());
            panic!();
        };

        self.file_index += 1;

        if self.file_index >= self.file_names.len() {
            self.file_index = 0;
        }

        if self.quad_side == QuadSide::Up {
            self.quad_side = QuadSide::Down;
        } else {
            self.quad_side = QuadSide::Up;
        }

    }

}

impl EventHandler for FlowState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let delta_duration = get_delta(_ctx);
        let delta = duration_to_f64(delta_duration);

        self.grid.update(delta);

        let fired = self.timer.fired().clone();

        for id in fired {
            if id == "swap_column" {
                if self.swapping_column.is_none() {
                    let elapsed = Instant::now().duration_since(self.last_ended_swap);

                    if elapsed > Duration::from_millis(LOAD_IMAGE_DELAY) {
                        self.load_image();
                        self.swapping_column = Some(0);
                    }
                } else if (self.swapping_column.unwrap() as f32) < (WIDTH / QUAD_SIZE) {
                    {
                        let sw = self.swapping_column.unwrap();
                        self.grid.swap_column_quads(sw, FLIP_VELOCITY);
                        // println!("swapping column {}", sw);
                    }
                    self.swapping_column = Some(self.swapping_column.unwrap() + 1);
                } else {
                    // println!("restart swap", );
                    self.swapping_column = None;
                    self.last_ended_swap = Instant::now();
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // graphics::clear(ctx);

        self.grid.draw(ctx)?;

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
            self.grid.flip_quad_right(x as f32, y as f32, FLIP_VELOCITY);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: i32,
        y: i32,
    ) {
        self.grid.flip_quad_right(x as f32, y as f32, FLIP_VELOCITY);
    }

}
