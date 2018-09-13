use std;
use std::ops::IndexMut;
use std::time::Instant;

use ggez::*;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseState};
use ggez::graphics::{DrawMode, Point2, Rect, Color};
use ggez::timer::{get_fps, get_delta, duration_to_f64};
use rand::*;

const SIZE : f32 = 20.0;
const MARGIN : f32 = 0.5;
const PI : f64 = std::f64::consts::PI;

lazy_static! {
    pub static ref UP_COLOR: Color = {
        Color::new(0.5, 0.0, 0.0, 1.0)
    };

    pub static ref DOWN_COLOR: Color = {
        Color::new(0.0, 0.5, 0.0, 1.0)
    };
}

pub struct Quad {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    // color_up: Color,
    // color_down: Color,
    angle: f64,
    va: f64,
}

/**
 * true -> positive
 * false -> negative
 */
fn sign(n: f64) -> bool {
    n >= 0.0
}

impl Quad {

    fn new(x: f32, y: f32, width: f32, height: f32, /*color_up: Color, color_down: Color,*/ angle: f64, va: f64) -> Quad {
        Quad{x: x, y: y, width: width, height: height, /*color_up: color_up, color_down: color_down,*/ angle: angle, va: va}
    }

    fn update(&mut self, delta: f64) {
        let prev_angle = self.angle;
        self.angle += self.va * delta;

        // if self.angle != 0.0 {
        //      println!("{} ({})", self.angle, self.angle / PI * 180.0);
        // }

        if sign(self.angle.sin()) != sign(prev_angle) {
            self.va = 0.0;
            self.angle = (self.angle / PI).round() * PI;
        }

    }

    fn draw(&self, _ctx: &mut Context, mb: &mut graphics::MeshBuilder) {
        let delta = self.angle.sin().abs() as f32 * SIZE / 2.0;
        // if delta != 0.0 {
        //     println!("{}", delta);
        //     println!("{} {} {} {} {}", self.x, Point2::new(self.x + delta + MARGIN, self.y + MARGIN),
        //         Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + MARGIN),
        //         Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + self.width - 2.0 * MARGIN),
        //         Point2::new(self.x + delta + MARGIN, self.y + self.width - 2.0 * MARGIN));
        // }

        mb.polygon(DrawMode::Fill, 
            &[
                Point2::new(self.x + delta + MARGIN, self.y + MARGIN),
                Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + MARGIN),
                Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + self.width - 2.0 * MARGIN),
                Point2::new(self.x + delta + MARGIN, self.y + self.width - 2.0 * MARGIN),
            ],
        );
    }

    // fn get_color(&self) -> &Color {
    //     if self.faced_up() {
    //         &self.color_up
    //     } else {
    //         &self.color_down
    //     }
    // }

    fn faced_up(&self) -> bool {
        self.angle.cos() >= 0.0
    }

}

pub struct FlowState {
    font: graphics::Font,
    quads: Vec<Quad>,
}

impl FlowState {

    pub fn new(font: graphics::Font) -> FlowState {
        let mut quads = Vec::new();
        let mut x = 0.0;
        let mut y = 0.0;

        while x < 800.0 {
            y = 0.0;
            while y < 600.0 {
                let quad = Quad::new(x, y, SIZE, SIZE, /*Color::new(0.5, 0.0, 0.0, 1.0), Color::new(0.0, 0.5, 0.0, 1.0),*/ 0.0, 0.0);
                quads.push(quad);
                y += SIZE;
            }
            x += SIZE;
        }

        FlowState{quads, font: font}
    }

    fn draw_fps(&self, ctx: &mut Context) -> GameResult<()> {
        let fps = get_fps(ctx).round();

        let text = graphics::Text::new(ctx, &format!("fps {}", fps), &self.font)?;

        let dest_point = graphics::Point2::new(10.0, 10.0);

        graphics::draw_ex(
                ctx,
                &text,
                graphics::DrawParam {
                    dest: dest_point,
                    color: Some(graphics::Color::from((255, 255, 255, 255))),
                    ..Default::default()
                },
        )
    }

}

impl EventHandler for FlowState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let delta_duration = get_delta(_ctx);
        let delta = duration_to_f64(delta_duration);

        let mut rng = thread_rng();

        let mut i: usize = 0;
        while i < self.quads.len() {
            let quad = self.quads.index_mut(i);
            quad.update(delta);
            i += 1;
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

        self.draw_fps(ctx)?;

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            Keycode::R => {
                let mut i: usize = 0;
                while i < self.quads.len() {
                    let quad = self.quads.index_mut(i);
                    quad.angle = 0.0;
                    quad.va = 0.0;
                    i += 1;
                }     
            },
            _ => (), // Do nothing
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        _x: i32,
        _y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        let mut i: usize = 0;
        while i < self.quads.len() {
            let quad = self.quads.index_mut(i);
            if _x as f32 >= quad.x && _x as f32 <= quad.x + quad.width {
                if _y as f32 >= quad.y && _y as f32 <= quad.y + quad.height {
                    // quad.angle = 0.0;
                    if quad.faced_up() {
                        quad.va = 2.0;
                    } else {
                        quad.va = -2.0;
                    }
                }
            }
            i += 1;
        }
    }

}
