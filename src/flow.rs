use std;
use std::ops::IndexMut;
use std::time::Duration;

use ggez::*;
use ggez::event::{EventHandler, Keycode, Mod, MouseState, MouseButton};
use ggez::graphics::{DrawMode, Point2, Color};
use ggez::timer::{get_delta, duration_to_f64};

use fps::*;
use sync_timer::*;
use quad::*;

const SIZE : f32 = 20.0;
const MARGIN : f32 = 0.5;
const FLIP_VELOCITY : f64 = 3.0;

lazy_static! {
    pub static ref UP_COLOR: Color = {
        Color::new(0.5, 0.0, 0.0, 1.0)
    };

    pub static ref DOWN_COLOR: Color = {
        Color::new(0.0, 0.5, 0.0, 1.0)
    };
}

pub struct FlowState {
    font: graphics::Font,
    quads: Vec<Quad>,
    ctrl: bool,
    swapping_column: usize,
    timer: SyncTimer,
}

impl FlowState {

    pub fn new(font: graphics::Font) -> FlowState {
        let mut quads = Vec::new();
        let mut x = 0.0;
        let mut y = 0.0;

        while x < 800.0 {
            y = 0.0;
            while y < 600.0 {
                let quad = Quad::new(x + MARGIN, y + MARGIN, SIZE - 2.0 * MARGIN, SIZE - 2.0 * MARGIN, /*Color::new(0.5, 0.0, 0.0, 1.0), Color::new(0.0, 0.5, 0.0, 1.0),*/ 0.0, 0.0);
                quads.push(quad);
                y += SIZE;
            }
            x += SIZE;
        }

        let mut timer = SyncTimer::new();
        timer.add(SyncEvent::new("swap_column", Duration::from_millis(200), true));

        FlowState{quads, font: font, ctrl: false, swapping_column: 0, timer: timer}
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
                if (self.swapping_column as f32) < (800.0 / SIZE) {
                    {
                        let sw = self.swapping_column;
                        self.swap_column_quads(sw);
                        // println!("swapping column {}", sw);
                    }
                    self.swapping_column += 1;
                } else {
                    // println!("restart swap", );
                    self.swapping_column = 0;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let mut mb = graphics::MeshBuilder::new();

            for quad in &self.quads {
                if !quad.faced_up() {
                    continue;
                }
                quad.draw(ctx, &mut mb);
            }

            let mesh = mb.build(ctx)?;
            graphics::set_color(ctx, *UP_COLOR)?;
            graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0)?;
        }

        {
            let mut mb = graphics::MeshBuilder::new();

            for quad in &self.quads {
                if quad.faced_up() {
                    continue;
                }
                quad.draw(ctx, &mut mb);
            }

            let mesh = mb.build(ctx)?;
            graphics::set_color(ctx, *DOWN_COLOR)?;
            graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0)?;
        }

        draw_fps(ctx, &self.font, graphics::Point2::new(10.0, 10.0), graphics::Color::from((255, 255, 255, 255)))?;

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
