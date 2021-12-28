use ggez::*;
use ggez::graphics::{Drawable, DrawParam, Image, Rect};
use image::RgbaImage;

use crate::ggez_utils::Point2;
use crate::transitions::transition::*;
use crate::velocity::*;

const VELOCITY: f32 = 15.0;

pub struct Slides {
    n_slides: u32,
    slides: Vec<Slide>,
    image: Option<Image>,
}

impl Slides {
    pub fn new(slides: u32) -> Slides {
        Slides { n_slides: slides, slides: Vec::new(), image: None }
    }
}

enum Direction {
    Right,
    Left,
}

struct Slide {
    i_width: u32,
    i_height: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    vx: f32,
    vy: i32,
    direction: Direction,
    ended: bool,
    velocity: Box<dyn Velocity>,
}

impl Slide {
    fn left(n_slides: u32, i_width: u32, i_height: u32, y: f32) -> Slide {
        Slide {
            i_width,
            i_height,
            x: i_width as f32,
            y,
            width: 0.0,
            height: i_height as f32 / n_slides as f32,
            vx: VELOCITY,
            vy: 0,
            direction: Direction::Left,
            ended: false,
            velocity: Slide::velocity(),
        }
    }

    fn right(n_slides: u32, i_width: u32, i_height: u32, y: f32) -> Slide {
        Slide {
            i_width,
            i_height,
            x: 0.0,
            y: y as f32,
            width: 0.0,
            height: i_height as f32 / n_slides as f32,
            vx: VELOCITY,
            vy: 0,
            direction: Direction::Right,
            ended: false,
            velocity: Slide::velocity(),
        }
    }

    fn velocity() -> Box<dyn Velocity> {
        Box::new(StepsVelocity::new(vec![1.0, 1.5, 2.0, 1.5, 1.0, 0.1]))
    }

    fn update(&mut self) {
        self.width += self.vx;

        match self.direction {
            Direction::Left => self.x -= self.vx,
            _ => {}
        }

        if self.vx < 0.0 {
            self.vx = 0.1;
        }

        if self.width >= self.i_width as f32 {
            self.ended = true;
            self.width = self.i_width as f32;
            self.x = 0.0;
        }

        self.vx = self.velocity.get_velocity(self.width / self.i_width as f32) * VELOCITY;
    }

    fn to_rect(&self) -> Rect {
        Rect::new(self.x / self.i_width as f32, self.y / self.i_height as f32, self.width / self.i_width as f32,
                  self.height / self.i_height as f32)
    }

    fn to_point(&self) -> Point2 {
        match self.direction {
            Direction::Left => Point2::new(0.0, self.y),
            _ => Point2::new(self.i_width as f32 - self.width, self.y)
        }
    }
}

impl Transition for Slides {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if self.slides.is_empty() {
            return Ok(false);
        }

        let mut ended = true;

        match &self.image {
            Some(i) => {
                for slide in &mut self.slides {
                    if !slide.ended {
                        &slide.update();

                        let draw_param =
                            DrawParam::default()
                                .src(slide.to_rect())
                                .dest(slide.to_point());

                        i.draw(ctx, draw_param)?;
                        ended = false;

                        //if slide.ended {
                        //    println!("{:?}", draw_param.src);
                        //    println!("{}", draw_param.dest);
                        //}
                    }
                }
            }
            None => {}
        }

        Ok(!ended)
    }

    fn update_image(&mut self, ctx: &mut Context, image: RgbaImage) {
        //println!("slides update_image");
        &self.slides.clear();

        let slide_height = image.height() as f32 / self.n_slides as f32;

        for i in 0..self.n_slides {
            let y = i as f32 * slide_height;

            let slide = if i % 2 == 0 {
                Slide::left(self.n_slides, image.width(), image.height(), y)
            } else {
                Slide::right(self.n_slides, image.width(), image.height(), y)
            };

            &self.slides.push(slide);
        }

        let i = Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &image.into_raw()).unwrap();

        self.image = Some(i);
    }
}

#[cfg(test)]
#[test]
fn test_left_slide() {
    let mut slide = Slide::left(8, 100, 50, 0.0);

    assert_eq!(false, slide.ended);

    slide.update();

    assert_eq!(false, slide.ended);
    assert_eq!(100.0 - VELOCITY, slide.x);

    assert_eq!(VELOCITY, slide.width);
}

#[test]
fn test_right_slide() {
    let mut slide = Slide::right(8, 100, 50, 0.0);

    assert_eq!(false, slide.ended);

    slide.update();

    assert_eq!(false, slide.ended);
    assert_eq!(0.0, slide.x);

    assert_eq!(VELOCITY, slide.width);
}
