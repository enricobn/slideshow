use ggez::graphics::{Canvas, DrawParam, Drawable, Image};
use ggez::*;

use crate::ggez_utils::Point2;

pub trait Transition {
    /// Should return true if the transition is still running.
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<bool>;

    fn update_image(&mut self, ctx: &mut Context, image: Image);
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
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<bool> {
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

    fn update_image(&mut self, _ctx: &mut Context, image: Image) {
        self.image = Some(image);
        self.ended = false;
    }
}
