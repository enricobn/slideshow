use ggez::graphics::{DrawParam, Drawable, Image, ImageFormat};
use ggez::*;
use image::RgbaImage;

use crate::ggez_utils::Point2;
use crate::transitions::transition::*;

const VELOCITY: f32 = 1.0;

pub struct Fade {
    image: Option<RgbaImage>,
    last_image: Option<RgbaImage>,
    ended: bool,
    alpha: f32,
}

impl Fade {
    pub fn new() -> Fade {
        Fade {
            image: None,
            last_image: None,
            ended: true,
            alpha: 0.0,
        }
    }
}

impl Fade {
    fn blend(&self, from: u8, to: u8) -> u8 {
        let f = from as f32 * (255.0 - self.alpha) / 255.0;
        let t = to as f32 * (self.alpha) / 255.0;
        (f + t) as u8
    }
}

impl Transition for Fade {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if !self.ended {
            match &self.image {
                Some(image) => {
                    let mut canvas = graphics::Canvas::from_frame(ctx, None);
                    //graphics::clear(ctx);
                    let mut ii = image.clone();

                    match &self.last_image {
                        Some(last_image) => {
                            for x in 0..ii.width() {
                                for y in 0..ii.height() {
                                    let color = ii.get_pixel_mut(x, y);
                                    let last_color = last_image.get_pixel(x, y);
                                    color[0] = self.blend(last_color[0], color[0]);
                                    color[1] = self.blend(last_color[1], color[1]);
                                    color[2] = self.blend(last_color[2], color[2]);
                                }
                            }

                            let i = Image::from_pixels(
                                ctx,
                                ii.as_raw(),
                                ImageFormat::Rgba8UnormSrgb,
                                image.width(),
                                image.height(),
                            );

                            let draw_param = DrawParam::default();
                            draw_param.dest(Point2::new(0.0, 0.0));

                            i.draw(&mut canvas, draw_param);
                        }
                        None => {
                            for x in 0..ii.width() {
                                for y in 0..ii.height() {
                                    let color = ii.get_pixel_mut(x, y);
                                    color[0] = self.blend(0, color[0]);
                                    color[1] = self.blend(0, color[1]);
                                    color[2] = self.blend(0, color[2]);
                                }
                            }
                            let i = Image::from_pixels(
                                ctx,
                                ii.as_raw(),
                                ImageFormat::Rgba8UnormSrgb,
                                image.width(),
                                image.height(),
                            );
                            let draw_param = DrawParam::default();
                            draw_param.dest(Point2::new(0.0, 0.0));

                            i.draw(&mut canvas, draw_param);
                        }
                    }
                    canvas.finish(ctx)?;
                }
                None => {}
            }
            self.alpha += VELOCITY;
            if self.alpha > 255.0 {
                self.alpha = 0.0;
                self.ended = true;
            }
            //println!("alpha {}", self.alpha as u8);
        }
        Ok(!self.ended)
    }

    fn update_image(&mut self, _ctx: &mut Context, image: RgbaImage) {
        self.last_image = self.image.clone();
        self.image = Some(image);
        self.ended = false;
    }
}
