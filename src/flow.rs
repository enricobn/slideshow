use std::ops::IndexMut;
use std::time::Instant;

use ggez::*;
use ggez::event::{self, EventHandler, Keycode, Mod, MouseState};
use ggez::graphics::{DrawMode, Point2, Rect, Color};
use ggez::timer::{get_fps, get_delta, duration_to_f64};
use rand::*;

pub struct Quad {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    angle: f32,
    va: f32,
}

impl Quad {

    fn new(x: f32, y: f32, width: f32, height: f32, color: Color, angle: f32, va: f32) -> Quad {
        Quad{x: x, y: y, width: width, height: height, color: color, angle: angle, va: va}
    }

    fn update(&mut self, delta: f64) {
        self.angle += self.va * delta as f32;

        /*if self.angle < 0.0 {
            self.angle = -self.y;
            self.vy = -self.vy;
        }*/

        /*if self.y + self.height > 600.0 {
            self.y = 600.0 - self.height;
            self.vy = -self.vy;
        }
        */
    }

    fn draw(&self, _ctx: &mut Context, mb: &mut graphics::MeshBuilder) {
        mb.polygon(DrawMode::Fill, 
            &[
                Point2::new(self.x, self.y),
                Point2::new(self.x + self.width, self.y),
                Point2::new(self.x + self.width, self.y + self.width),
                Point2::new(self.x, self.y + self.width),
            ],
        );

        // graphics::rectangle(_ctx, DrawMode::Fill, Rect::new(
        //             self.x, 
        //             self.y, 
        //             self.width, 
        //             self.height
        //         ))?;
        // mb.build(_ctx)
    }

}

pub struct FlowState {
    font: graphics::Font,
    quads: Vec<Quad>,
}

const SIZE : f32 = 20.0;

impl FlowState {

    pub fn new(font: graphics::Font) -> FlowState {
        let mut quads = Vec::new();
        let mut x = 0.0;
        let mut y = 0.0;

        while x < 800.0 {
            y = 0.0;
            while y < 600.0 {
                let quad = Quad::new(x, y, SIZE, SIZE, Color::new(0.5, 0.0, 0.0, 1.0), 0.0, 0.0);
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

        let mut mb = graphics::MeshBuilder::new();

        for quad in &self.quads {
            if quad.angle != 0.0 {
                continue;
            }

            quad.draw(ctx, &mut mb);
        }

        let mesh = mb.build(ctx)?;
        graphics::set_color(ctx, graphics::Color::from((100, 0, 0, 255)))?;
        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0)?;

        let mut mb = graphics::MeshBuilder::new();
        for quad in &self.quads {
            if quad.angle == 0.0 {
                continue;
            }
            
            quad.draw(ctx, &mut mb);
        }

        let mesh = mb.build(ctx)?;
        graphics::set_color(ctx, graphics::Color::from((0, 0, 0, 255)))?;
        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0)?;

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
                    // quad.color = graphics::Color::from((0, 0, 0, 255));
                    quad.angle = 1.0;
                }
            }
            i += 1;
        }
    }

}
