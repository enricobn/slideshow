use ggez::*;
use ggez::graphics::{Point2,Image,Drawable};
use image::RgbaImage;

pub trait Transition {

    /// Should return true if the transition is still running. 
    /// It should return true even in the final step of the transition since if false then graphics is not updated.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool>;

    fn update(&mut self, ctx: &mut Context, image: RgbaImage);

}

pub struct SimpleTransition {
    image: Option<Image>,
    ended: bool,
}

impl SimpleTransition {

    pub fn new() -> SimpleTransition {
        SimpleTransition{image: None, ended: true}
    }
}

impl Transition for SimpleTransition {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if !self.ended {
            self.ended = true;
            match &self.image {
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
        let i = Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &image.into_raw()).unwrap();
        self.image = Some(i);
        self.ended = false;
    }

}