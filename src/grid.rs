use std::slice::IterMut;

use graphics::*;
use image;

use quad::*;


#[derive(PartialEq)]
pub enum QuadSide {
    Up,
    Down,
}

pub struct Grid {
    background_color: Color,
    quad_size: f32,
    quads: Vec<Quad>,
}

impl Grid {

    pub fn new(quad_size: f32, background_color: Color) -> Grid {
        Grid {quad_size: quad_size, quads: Vec::new(), background_color: background_color}
    }

    pub fn find_quad(&mut self, x: f32, y: f32) -> Option<&mut Quad> {
        for quad in self.quads.iter_mut() {
            if x >= quad.x && x <= quad.x + quad.width {
                if y >= quad.y && y  <= quad.y + quad.height {
                    return Some(quad);
                }
            }
        }
        None
    }

    pub fn swap_column_quads(&mut self, column: usize, flip_speed: f64) {
        let x = column as f32 * self.quad_size + self.quad_size / 2.0;
        for quad in self.quads.iter_mut() {
            if x >= quad.x && x <= quad.x + quad.width {
                quad.flip_right(flip_speed);
                // println!("swapping {}", x);
            }
        }
    }

    pub fn add(&mut self, quad: Quad) {
        self.quads.push(quad);
    }

    pub fn iter_mut(&mut self) -> IterMut<Quad> {
        self.quads.iter_mut()
    }

    pub fn load_image(&mut self, img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, side: &QuadSide) {
        let (img_width, img_height) = img.dimensions();

        for quad in self.quads.iter_mut() {
            let ix = (quad.x / self.quad_size).round() as u32;
            let iy = (quad.y / self.quad_size).round() as u32;

            let color = if ix >= img_width || iy >= img_height {
                self.background_color
            } else {
                pixel_to_color(img.get_pixel(ix, iy))
            };

            if *side == QuadSide::Down {
                quad.set_down_color(color);
            } else {
                quad.set_up_color(color);
            }
        }
    }

}

fn pixel_to_color(pixel: &image::Rgba<u8>) -> Color {
    Color::new(pixel.data[0] as f32 / 255.0, pixel.data[1] as f32 / 255.0, pixel.data[2] as f32 / 255.0, 
                        pixel.data[3] as f32 / 255.0)
}