use ggez::graphics::{DrawParam, Drawable, Image, ImageFormat, Rect};
use ggez::*;
use image::RgbaImage;
use rand::Rng;

use crate::ggez_utils::Point2;
use crate::transitions::transition::*;

const V_QUADS: u16 = 10;

pub struct Quads {
    quads: Vec<Quad>,
    image: Option<RgbaImage>,
}

impl Quads {
    pub fn new() -> Quads {
        Quads {
            quads: Vec::new(),
            image: None,
        }
    }
}

#[derive(Debug)]
struct Quad {
    x: u16,
    y: u16,
}

impl Quad {
    pub fn new(x: u16, y: u16) -> Quad {
        Quad { x, y }
    }
}

impl Transition for Quads {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if self.quads.is_empty() {
            return Ok(false);
        }

        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0usize, self.quads.len());
        let removed = self.quads.remove(index);
        //println!("draw quad {:?}", removed);

        match &self.image {
            Some(image) => {
                let quad_size = image.height() as f32 / V_QUADS as f32;
                let quad_x = removed.x as f32 * quad_size;
                let quad_y = removed.y as f32 * quad_size;

                let x = quad_x / image.width() as f32;
                let y = quad_y / image.height() as f32;
                let width = quad_size / image.width() as f32;
                let height = quad_size / image.height() as f32;

                //println!("{},{} -> {},{},{},{}", quad_x, quad_y, x, y, width, height);

                let draw_param = DrawParam::default()
                    .src(Rect::new(x, y, width, height))
                    .dest(Point2::new(quad_x, quad_y));

                let i = Image::from_pixels(
                    ctx,
                    image.as_raw(),
                    ImageFormat::Rgba8UnormSrgb,
                    image.width(),
                    image.height(),
                );

                i.draw(&mut canvas, draw_param);
            }
            None => {}
        }

        canvas.finish(ctx)?;

        Ok(true)
    }

    fn update_image(&mut self, _ctx: &mut Context, image: RgbaImage) {
        self.quads.clear();

        let quad_size = image.height() as f32 / V_QUADS as f32;

        let h_quads = (image.width() as f32 / quad_size) as u16;
        let v_quads = (image.height() as f32 / quad_size) as u16;

        for x in 0..h_quads {
            for y in 0..v_quads {
                let quad = Quad::new(x, y);
                //println!("quad {:?}", &quad);
                self.quads.push(quad);
            }
        }

        self.image = Some(image);
    }
}
