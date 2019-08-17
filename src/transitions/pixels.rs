use ggez::*;
use ggez::graphics::{Color, DrawMode, DrawParam, Drawable, MeshBuilder, Rect};
use image::RgbaImage;
use rand::Rng;

use ggez_utils::{draw_rect, Point2};
use transitions::transition::*;

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
        Pixel{x: x, y: y}
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

        for _i in 0..2_000 {
            if self.pixels.is_empty() {
                break;
            }

            changed = true;

            let index = rng.gen_range(0usize, self.pixels.len());
            let removed = &self.pixels.remove(index);

            match &self.image {
                Some(i) => {
                    let p = i.get_pixel(removed.x as u32, removed.y as u32);
                    let c = pixel_to_color(p);

                    let rect = Rect::new(removed.x as f32, removed.y as f32, 1.0, 1.0);

                    mesh_builder.rectangle(DrawMode::fill(), rect, c);
                }
                None => {}
            }
        }

        let mesh = mesh_builder.build(ctx)?;

        let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

        mesh.draw(ctx, param)?;

        Ok(changed)

    }

    fn update(&mut self, ctx: &mut Context, image: RgbaImage) {
        &self.pixels.clear();
        for x in 0..image.width() {
            for y in 0..image.height() {
                &self.pixels.push(Pixel::new(x as u16, y as u16));
            }
        }
        
        self.image = Some(image);
    }

}

fn pixel_to_color(pixel: &image::Rgba<u8>) -> Color {
    Color::new(pixel.data[0] as f32 / 255.0, pixel.data[1] as f32 / 255.0, pixel.data[2] as f32 / 255.0, 
                        pixel.data[3] as f32 / 255.0)
}