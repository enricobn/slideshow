use ggez::*;
use ggez::event::{EventHandler, Keycode, Mod, MouseState, MouseButton};
use ggez::graphics::{Color,Point2,Image,Drawable};
use ggez::timer::{get_delta, duration_to_f64};
use ggez::timer::{get_fps};
use image;

pub trait Transition {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;

    fn update(&mut self, image: Image);

}

pub struct SimpleTransition {
    back_image: Option<Image>,
    new_image: Option<Image>
}

impl SimpleTransition {

    pub fn new() -> SimpleTransition {
        SimpleTransition{back_image: None, new_image: None}
    }
}

impl Transition for SimpleTransition {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match &self.new_image {
            Some(i) => i.draw(ctx, Point2::new(0.0, 0.0), 0.0),
            None => Ok(())
        }
    }

    fn update(&mut self, image: Image) {
        //self.back_image = self.new_image;
        self.new_image = Some(image);
    }

}