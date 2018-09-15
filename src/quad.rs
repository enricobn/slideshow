use ggez::*;
use ggez::graphics::{DrawMode, Point2, Color};

use std::f64::consts::PI;
use globals::*;

pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    // color_up: Color,
    // color_down: Color,
    pub angle: f64,
    pub va: f64,
}

impl Quad {

    pub fn new(x: f32, y: f32, width: f32, height: f32, /*color_up: Color, color_down: Color,*/ angle: f64, va: f64) -> Quad {
        Quad{x: x, y: y, width: width, height: height, /*color_up: color_up, color_down: color_down,*/ angle: angle, va: va}
    }

    pub fn update(&mut self, delta: f64) {
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

    pub fn draw(&self, _ctx: &mut Context, mb: &mut graphics::MeshBuilder) {
        let delta = self.angle.sin().abs() as f32 * self.height / 2.0;
        // if delta != 0.0 {
        //     println!("{}", delta);
        //     println!("{} {} {} {} {}", self.x, Point2::new(self.x + delta + MARGIN, self.y + MARGIN),
        //         Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + MARGIN),
        //         Point2::new(self.x + self.width - delta - 2.0 * MARGIN, self.y + self.width - 2.0 * MARGIN),
        //         Point2::new(self.x + delta + MARGIN, self.y + self.width - 2.0 * MARGIN));
        // }

        mb.polygon(DrawMode::Fill, 
            &[
                Point2::new(self.x + delta, self.y),
                Point2::new(self.x + self.width - delta, self.y),
                Point2::new(self.x + self.width - delta, self.y + self.width),
                Point2::new(self.x + delta, self.y + self.width),
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

    pub fn faced_up(&self) -> bool {
        self.angle.cos() >= 0.0
    }

}
