use ggez::*;
use ggez::graphics::{Point2,Image,Drawable};
use image::RgbaImage;

pub trait Transition {

    /// Should return false if the transition is ended.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool>;

    fn update(&mut self, ctx: &mut Context, image: RgbaImage);

}

pub struct SimpleTransition {
    back_image: Option<Image>,
    new_image: Option<Image>,
    ended: bool,
}

impl SimpleTransition {

    pub fn new() -> SimpleTransition {
        SimpleTransition{back_image: None, new_image: None, ended: true}
    }
}

impl Transition for SimpleTransition {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if !self.ended {
            self.ended = true;
            match &self.new_image {
                Some(i) => {
                    graphics::clear(ctx);
                    i.draw(ctx, Point2::new(0.0, 0.0), 0.0)?;
                    Ok(true)
                },
                None => Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn update(&mut self, ctx: &mut Context, image: RgbaImage) {
        //self.back_image = self.new_image;
        let i = Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &image.into_raw()).unwrap();
        self.new_image = Some(i);
        self.ended = false;
    }

}