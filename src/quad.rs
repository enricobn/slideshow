use ggez::*;
use ggez::graphics::{DrawMode, Point2, Color, Rect};

use std::f64::consts::PI;
use globals::*;

pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    up_color: Color,
    down_color: Color,
    angle: f64,
    va: f64,
    updated: bool,
}

impl Quad {

    pub fn new(x: f32, y: f32, width: f32, height: f32, up_color: Color, down_color: Color, angle: f64, va: f64) -> Quad {
        Quad{x: x, y: y, width: width, height: height, up_color: up_color, down_color: down_color, angle: angle, va: va, updated: true}
    }

    pub fn update(&mut self, delta: f64) {
        if self.va == 0.0 {
            return;
        }

        let prev_angle = self.angle;
        self.angle += self.va * delta;

        // if self.angle != 0.0 {
        //      println!("{} ({})", self.angle, self.angle / PI * 180.0);
        // }

        let actual_sign = sign(self.angle.sin());
        if actual_sign != sign(prev_angle.sin()) {
            self.angle = (self.angle / PI).round() * PI;
            self.va = 0.0;
        }

        self.updated = true;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.updated {
            return Ok(());
        }
        let delta = self.angle.sin().abs() as f32 * self.height / 2.0;
        // if delta != 0.0 {
        //     println!("{}", delta);
        //     println!("{} {} {} {} {}", self.x, Point2::new(self.x + delta + MARGIN, self.y + MARGIN),
        //         Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + MARGIN),
        //         Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + self.width - 2.0 * MARGIN),
        //         Point2::new(self.x + delta + MARGIN, self.y + self.width - 2.0 * MARGIN));
        // }

        // mb.polygon(DrawMode::Fill, 
        //     &[
        //         Point2::new(self.x + delta, self.y),
        //         Point2::new(self.x + self.width - delta, self.y),
        //         Point2::new(self.x + self.width - delta, self.y + self.width),
        //         Point2::new(self.x + delta, self.y + self.width),
        //     ],
        // );

        self.updated = false;

        graphics::set_color(ctx, *self.get_color())?;

        graphics::rectangle(ctx, DrawMode::Fill, 
            Rect::new(self.x + delta, self.y, self.width - 2.0 * delta, self.height))
    }

    pub fn draw_bk(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::rectangle(ctx, DrawMode::Fill, 
            Rect::new(self.x, self.y, self.width, self.height))
    }


    fn get_color(&self) -> &Color {
        if self.faced_up() {
            &self.up_color
        } else {
            &self.down_color
        }
    }

    pub fn faced_up(&self) -> bool {
        self.angle.cos() >= 0.0
    }

    pub fn flip_right(&mut self, velocity: f64) {
        self.va = velocity;
        // since we stop rotation when sin sign changes, I want to be sure that the actual sin sign is correct.
        self.angle += 0.00001;
    }

    pub fn flip_left(&mut self, velocity: f64) {
        self.va = -velocity;
        // since we stop rotation when sin sign changes, I want to be sure that the actual sin sign is correct.
        self.angle -= 0.00001;
    }

    pub fn is_updated(&self) -> bool {
        self.updated
    }

    pub fn set_up_color(&mut self, color: Color) {
        self.up_color = color;
    }

    pub fn set_down_color(&mut self, color: Color) {
        self.down_color = color;
    }

    fn flipping(&self) -> bool {
        self.va != 0.0
    }

}

#[cfg(test)]

#[test]
fn flip() {
    let mut quad = Quad::new(0.0, 0.0, 100.0, 100.0, Color::new(0.0, 0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0, 0.0), 0.0, 0.0);

    // first flip
    quad.flip_right(2.0);

    while quad.flipping() {
        quad.update(0.1);
    }

    // second flip
    quad.flip_right(2.0);
    quad.update(0.1);

    assert_eq!(true, quad.flipping());
    
    while quad.flipping() {
        quad.update(0.1);
    }

    // third flip
    quad.flip_right(2.0);
    quad.update(0.1);

    assert_eq!(true, quad.flipping());
}
