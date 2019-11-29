use ggez::*;
use ggez::graphics::{BLACK, Drawable, DrawParam, Image};
use image::RgbaImage;

use ggez_utils::Point2;

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
        SimpleTransition { image: None, ended: true }
    }
}

impl Transition for SimpleTransition {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if !self.ended {
            self.ended = true;
            match &self.image {
                Some(i) => {
                    graphics::clear(ctx, BLACK);
                    let param = DrawParam::new().dest(Point2::new(0.0, 0.0));
                    i.draw(ctx, param)?;
                }
                None => {}
            }
        }
        Ok(false)
    }

    fn update_image(&mut self, ctx: &mut Context, image: RgbaImage) {
        let i = Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &image.into_raw()).unwrap();
        self.image = Some(i);
        self.ended = false;
    }
}