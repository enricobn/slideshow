use ggez::*;
use transition::*;
use ggez::graphics::{Color,Rect,DrawMode};
use image::RgbaImage;
use rand::Rng;

const QUAD_SIZE : u32 = 31;

pub struct Quads {
    quads: Vec<Quad>,
    image: Option<RgbaImage>
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
                for x in 0..QUAD_SIZE {
                    for y in 0..QUAD_SIZE {
                        if quad_x + x < i.width() && quad_y + y < i.height() {
                            let p = i.get_pixel(quad_x + x as u32, quad_y + y as u32);
                            let c = pixel_to_color(p);
                            graphics::set_color(ctx, c)?;
                            
                            graphics::rectangle(ctx, DrawMode::Fill, 
                                Rect::new((quad_x + x) as f32, (quad_y + y) as f32, 1.0, 1.0))?;
                        }
                    }
                }
            }
            None => {}
        }

        Ok(true)

    }

    fn update(&mut self, image: RgbaImage) {
        &self.quads.clear();

        let h_quads = image.width() / QUAD_SIZE; 
        let v_quads = image.height() / QUAD_SIZE;

        for x in 0..h_quads {
            for y in 0..v_quads {
                &self.quads.push(Quad::new(x as u16, y as u16));
            }
        }
        
        self.image = Some(image);
    }

}

fn pixel_to_color(pixel: &image::Rgba<u8>) -> Color {
    Color::new(pixel.data[0] as f32 / 255.0, pixel.data[1] as f32 / 255.0, pixel.data[2] as f32 / 255.0, 
                        pixel.data[3] as f32 / 255.0)
}