use ggez::graphics::{DrawParam, Drawable, Image, ImageFormat};
use ggez::*;
use image::RgbaImage;

use crate::ggez_utils::Point2;

pub trait Transition {
    /// Should return true if the transition is still running.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool>;

    fn update_image(&mut self, ctx: &mut Context, image: RgbaImage);
}

pub struct SimpleTransition {
    image: Option<Image>,
    ended: bool,
}

impl SimpleTransition {
    pub fn new() -> SimpleTransition {
        SimpleTransition {
            image: None,
            ended: true,
        }
    }
}

impl Transition for SimpleTransition {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if !self.ended {
            self.ended = true;
            match self.image {
                Some(ref i) => {
                    let mut canvas = graphics::Canvas::from_frame(ctx, None);
                    let param = DrawParam::new().dest(Point2::new(0.0, 0.0));
                    i.draw(&mut canvas, param);
                    canvas.finish(ctx)?;
                }
                None => {}
            }
        }
        Ok(false)
    }

    fn update_image(&mut self, ctx: &mut Context, image: RgbaImage) {
        let i = Image::from_pixels(
            ctx,
            image.as_raw(),
            ImageFormat::Rgba8UnormSrgb,
            image.width(),
            image.height(),
        );
        self.image = Some(i);
        self.ended = false;
    }
}
