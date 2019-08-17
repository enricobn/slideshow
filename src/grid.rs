use std::slice::IterMut;

use graphics::*;
use ggez::*;
use image;

use quad::*;

#[derive(PartialEq)]
pub enum QuadSide {
    Up,
    Down,
}

pub struct Grid {
    width: f32, 
    height: f32,
    background_color: Color,
    quad_size: f32,
    quads: Vec<Quad>,
}

impl Grid {

    pub fn new(width: f32, height: f32, quad_size: f32, quad_margin: f32, background_color: Color) -> Grid {
        let mut grid = Grid {width: width, height: height, quad_size: quad_size, quads: Vec::new(), background_color: background_color};

        let mut x = 0.0;

        while x < width {
            let mut y = 0.0;
            while y < height {
                let quad = Quad::new(x + quad_margin, y + quad_margin, quad_size - 2.0 * quad_margin, quad_size - 2.0 * quad_margin, background_color, background_color, 0.0, 0.0);
                grid.add(quad);
                y += quad_size;
            }
            x += quad_size;
        }

        grid
    }

    pub fn update(&mut self, delta: f64) {
        for quad in self.iter_mut() {
            quad.update(delta);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for quad in self.quads.iter_mut() {
            if !quad.is_updated() {
               continue; 
            }
            quad.draw_bk(ctx, &self.background_color)?;
        }

        for quad in self.quads.iter_mut() {
            quad.draw(ctx)?;
        }

        Ok(())
    }

    fn find_quad(&mut self, x: f32, y: f32) -> Option<&mut Quad> {
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

    pub fn load_image(&mut self, file: &str, side: &QuadSide) -> Result<(), image::ImageError> {//, img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, side: &QuadSide) {
        let img = self.load_and_resize_image(file)?;
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

        Ok(())
    }

    fn load_and_resize_image(&self, file: &str) -> image::ImageResult<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
        let maybe_img = image::open(&file)?;
        // println!("file opened", );

        let img = maybe_img.to_rgba();

        let (width, height) = img.dimensions();
        // println!("img {}x{}", width, height);

        let grid_width = self.width / self.quad_size;
        let grid_height = self.height / self.quad_size;

        // println!("grid {}x{}", grid_width, grid_height);

        let width_coeff = width as f32 / grid_width;
        let height_coeff = height as f32 / grid_height;

        let coeff = width_coeff.max(height_coeff);

        let new_img = image::imageops::resize(&img, 
            (width as f32 / coeff) as u32, 
            (height as f32 / coeff) as u32, 
            image::FilterType::Gaussian);
        
        // let (new_width, new_height) = new_img.dimensions();
        // println!("new img {}x{}", new_width, new_height);

        Ok(new_img)

    }

    pub fn flip_quad_right(&mut self, x: f32, y: f32, speed: f64) {
        match self.find_quad(x, y) {
            Some(quad) => quad.flip_right(speed),
            _ => ()
        };
    }

}

fn pixel_to_color(pixel: &image::Rgba<u8>) -> Color {
    Color::new(pixel.data[0] as f32 / 255.0, pixel.data[1] as f32 / 255.0, pixel.data[2] as f32 / 255.0, 
                        pixel.data[3] as f32 / 255.0)
}