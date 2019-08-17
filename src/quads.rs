use ggez::*;
use ggez::graphics::{Drawable, DrawParam, Image, Rect};
use image::RgbaImage;
use rand::Rng;

use ggez_utils::Point2;
use transition::*;

const QUAD_SIZE : u32 = 30;

pub struct Quads {
    quads: Vec<Quad>,
    image: Option<Image>
}

impl Quads {

    pub fn new() -> Quads {
        Quads{quads: Vec::new(), image: None}
    }

}

struct Quad {
    x: u16,
    y: u16
}

impl Quad {

    pub fn new(x: u16, y: u16) -> Quad {
        Quad{x: x, y: y}
    }
}

impl Transition for Quads {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        let mut rng = rand::thread_rng();

        if self.quads.is_empty() {
            return Ok(false);
        }

        let index = rng.gen_range(0usize, self.quads.len());
        let removed = &self.quads.remove(index);

        match &self.image {
            Some(i) => {
                let quad_x = removed.x as u32 * QUAD_SIZE;
                let quad_y = removed.y as u32 * QUAD_SIZE;

                let mut draw_param = DrawParam::default();
                draw_param.src(Rect::new(quad_x as f32 / i.width() as f32, quad_y as f32 / i.height() as f32, QUAD_SIZE as f32 / i.width() as f32, QUAD_SIZE as f32 / i.height() as f32));
                draw_param.dest(Point2::new(quad_x as f32, quad_y as f32));

                i.draw(ctx, draw_param)?;
            }
            None => {}
        }

        Ok(true)

    }

    fn update(&mut self, ctx: &mut Context, image: RgbaImage) {
        &self.quads.clear();

        let h_quads = image.width() / QUAD_SIZE; 
        let v_quads = image.height() / QUAD_SIZE;

        for x in 0..h_quads {
            for y in 0..v_quads {
                &self.quads.push(Quad::new(x as u16, y as u16));
            }
        }
        
        let i = Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &image.into_raw()).unwrap();

        self.image = Some(i);
    }

}