use ggez::graphics::{Color, DrawMode, DrawParam, Drawable, Mesh, MeshBuilder, Rect};
use ggez::*;
use image::RgbaImage;
use rand::Rng;

use crate::ggez_utils::Point2;
use crate::transitions::transition::*;

pub struct Pixels {
    pixels: Vec<Pixel>,
    image: Option<RgbaImage>,
}

impl Pixels {
    pub fn new() -> Pixels {
        Pixels {
            pixels: Vec::new(),
            image: None,
        }
    }
}

struct Pixel {
    x: u16,
    y: u16,
}

impl Pixel {
    pub fn new(x: u16, y: u16) -> Pixel {
        Pixel { x, y }
    }
}

impl Transition for Pixels {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if self.pixels.is_empty() {
            return Ok(false);
        }

        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        let mut mesh_builder = MeshBuilder::new();

        match self.image {
            Some(ref image) => {
                let count = image.width() * image.height() / 300;

                for _i in 0..count {
                    if let Some(pixel) = self.pixels.pop() {
                        let pixel_rgba = image.get_pixel(pixel.x as u32, pixel.y as u32);
                        let pixel_color = pixel_to_color(pixel_rgba);

                        let rect = Rect::new(pixel.x as f32, pixel.y as f32, 1.0, 1.0);

                        mesh_builder.rectangle(DrawMode::fill(), rect, pixel_color)?;
                    } else {
                        break;
                    }
                }
            }
            None => {}
        }

        let mesh = Mesh::from_data(ctx, mesh_builder.build());

        let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

        mesh.draw(&mut canvas, param);

        canvas.finish(ctx)?;

        Ok(true)
    }

    fn update_image(&mut self, _ctx: &mut Context, image: RgbaImage) {
        self.pixels.clear();

        for x in 0..image.width() {
            for y in 0..image.height() {
                self.pixels.push(Pixel::new(x as u16, y as u16));
            }
        }

        let mut rng = rand::thread_rng();

        rng.shuffle(&mut self.pixels);

        self.image = Some(image);
    }
}

fn pixel_to_color(pixel: &image::Rgba<u8>) -> Color {
    Color::new(
        pixel.0[0] as f32 / 255.0,
        pixel.0[1] as f32 / 255.0,
        pixel.0[2] as f32 / 255.0,
        pixel.0[3] as f32 / 255.0,
    )
}
