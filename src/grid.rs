use std::slice::IterMut;

use quad::*;

pub struct Grid {
    quad_size: f32,
    quads: Vec<Quad>,
}

impl Grid {

    pub fn new(quad_size: f32) -> Grid {
        Grid {quad_size: quad_size, quads: Vec::new()}
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

}