use ggez::*;
use ggez::event::{EventHandler, Keycode, Mod, MouseState, MouseButton};
use ggez::graphics::{Color,Point2,Image,Drawable};
use ggez::timer::{get_delta, duration_to_f64};
use ggez::timer::{get_fps};
use image::RgbaImage;

pub trait Transition {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool>;

    fn update(&mut self, image: RgbaImage);

}

pub struct SimpleTransition {
    back_image: Option<RgbaImage>,
    new_image: Option<RgbaImage>,
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
                    // TODO
                    //let image = Image::from_rgba8(ctx, i.width() as u16, i.height() as u16, &i.into_raw()).unwrap();
                    //image.draw(ctx, Point2::new(0.0, 0.0), 0.0)?;
                    Ok(true)
                },
                None => Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn update(&mut self, image: RgbaImage) {
        //self.back_image = self.new_image;
        self.new_image = Some(image);
        self.ended = false;
    }

}