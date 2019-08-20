use ggez::*;
use ggez::graphics::{Color, DrawMode, DrawParam, Drawable, MeshBuilder, Rect};
use image::{RgbaImage, GenericImage};
use rand::Rng;

use ggez_utils::{draw_rect, Point2};
use transitions::transition::*;
use std::time::Instant;

pub struct Pixels {
    pixels: Vec<Pixel>,
    image: Option<RgbaImage>,
}

impl Pixels {

    pub fn new() -> Pixels {
        Pixels{pixels: Vec::new(), image: None}
    }

}

struct Pixel {
    x: u16,
    y: u16
}

impl Pixel {

    pub fn new(x: u16, y: u16) -> Pixel {
        Pixel{x, y }
    }
}

impl Transition for Pixels {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {

        if self.pixels.is_empty() {
            return Ok(false);
        }

        let mut rng = rand::thread_rng();

        let mut changed = false;

        let mut mesh_builder = MeshBuilder::new();

        match &self.image {
            Some(image) => {

                changed = !self.pixels.is_empty();

                let count = image.width() * image.height() / 300;

                for _i in 0..count {
                    if let Some(pixel) = self.pixels.pop() {
                        let pixel_rgba = image.get_pixel(pixel.x as u32, pixel.y as u32);
                        let pixel_color = pixel_to_color(pixel_rgba);

                        let rect = Rect::new(pixel.x as f32, pixel.y as f32, 1.0, 1.0);

                        mesh_builder.rectangle(DrawMode::fill(), rect, pixel_color);
                    } else {
                        break;
                    }
                }
            }
            None => {}
        }

        let mesh = mesh_builder.build(ctx)?;

        let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

        mesh.draw(ctx, param)?;

        Ok(changed)
    }

    fn update_image(&mut self, ctx: &mut Context, image: RgbaImage) {
        &self.pixels.clear();

        for x in 0..image.width() {
            for y in 0..image.height() {
                &self.pixels.push(Pixel::new(x as u16, y as u16));
            }
        }

        let mut rng = rand::thread_rng();

        rng.shuffle(&mut self.pixels);
        
        self.image = Some(image);
    }

}

fn pixel_to_color(pixel: &image::Rgba<u8>) -> Color {
    Color::new(pixel.data[0] as f32 / 255.0, pixel.data[1] as f32 / 255.0, pixel.data[2] as f32 / 255.0, 
                        pixel.data[3] as f32 / 255.0)
}