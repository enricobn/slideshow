use ggez::*;
use transition::*;
use ggez::graphics::{Rect,DrawParam,Image,Point2,Drawable};
use image::RgbaImage;
use rand::Rng;

const SLIDES : u32 = 8;
const VELOCITY : i32 = 8;

pub struct Slides {
    slides: Vec<Slide>,
    image: Option<Image>
}

impl Slides {

    pub fn new() -> Slides {
        Slides{slides: Vec::new(), image: None}
    }

}

struct Slide {
    i_width: u32,
    i_height: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    vx: i32,
    vy: i32,
    ended: bool
}

impl Slide {

    fn from_left(i_width: u32, i_height: u32, y: u32) -> Slide {
        Slide{i_width: i_width, i_height: i_height, x: i_width, y: y, width: 0, height: i_height / SLIDES, vx: VELOCITY, vy: 0, ended:false}
    }

    fn from_right(i_width: u32, i_height: u32, y: u32) -> Slide {
        Slide{i_width: i_width, i_height: i_height, x: 0, y: y, width: 0, height: i_height / SLIDES, vx: -VELOCITY, vy: 0, ended:false}
    }

    fn update(&mut self) {
        if self.vx > 0 {
            self.width = (self.width as i32 + self.vx) as u32;
            self.x = (self.x as i32 - self.vx) as u32;
        } else {
            self.width = (self.width as i32 - self.vx) as u32;
        }

        if self.width >= self.i_width {
            self.ended = true;
            self.width = self.i_width;
            self.x = 0;
        }
    }

    fn to_rect(&self) -> Rect {
        Rect::new(self.x as f32 / self.i_width as f32, self.y as f32 / self.i_height as f32, self.width as f32 / self.i_width as f32, 
            self.height as f32 / self.i_height as f32)
    }

    fn to_point(&self) -> Point2 {
        if self.vx > 0 {
            Point2::new(0.0, self.y as f32 as f32)
        } else {
            Point2::new((self.i_width - self.width) as f32, self.y as f32)
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
                for mut slide in &mut self.slides {
                    if !slide.ended {
                        &slide.update();

                        let mut draw_param = DrawParam::default();
                        draw_param.src = slide.to_rect();
                        draw_param.dest = slide.to_point();

                        i.draw_ex(ctx, draw_param)?;
                        ended = false;
                    }
                }
            }
            None => {}
        }

        Ok(!ended)

    }

    fn update(&mut self, ctx: &mut Context, image: RgbaImage) {
        &self.slides.clear();

        let slide_height = image.height() as f32 / SLIDES as f32;

        for i in 0..SLIDES {
            let y = (i as f32 * slide_height) as u32;
            if i % 2 == 0 {
                &self.slides.push(Slide::from_left(image.width(), image.height(), y));
            } else {
                &self.slides.push(Slide::from_right(image.width(), image.height(), y));
            }
        }
        
        let i = Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &image.into_raw()).unwrap();

        self.image = Some(i);
    }

}

#[cfg(test)]

#[test]
fn test_left_slide() {
    let mut slide = Slide::from_left(100, 50, 0);

    assert_eq!(false, slide.ended);

    slide.update();

    assert_eq!(false, slide.ended);
    assert_eq!((100 - VELOCITY) as u32, slide.x);

    assert_eq!(VELOCITY as u32, slide.width);
}

#[test]
fn test_right_slide() {
    let mut slide = Slide::from_right(100, 50, 0);

    assert_eq!(false, slide.ended);

    slide.update();

    assert_eq!(false, slide.ended);
    assert_eq!(0, slide.x);

    assert_eq!(VELOCITY as u32, slide.width);
}
