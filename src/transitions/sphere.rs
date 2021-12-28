use gfx::*;
use ggez::*;
use ggez::graphics::{Color, Drawable, DrawParam, Image};
use image::RgbaImage;

use crate::ggez_utils::Point2;
use crate::transitions::transition::Transition;

// Define the input struct for our shader.
gfx_defines! {
    constant Dim {
        rate: f32 = "u_Rate",
        center_x: f32 = "center_x",
        center_y: f32 = "center_y",
        radius: f32 = "radius",
        aspect_ratio: f32 = "aspectRatio",
        refractive_index: f32 = "refractiveIndex",
    }
}

pub struct Sphere {
    image: Option<Image>,
    ended: bool,
    shader: Option<graphics::Shader<Dim>>,
    dim: Dim,
}

impl Sphere {
    pub fn new() -> Sphere {
        let dim = Dim {
            rate: 1.0,
            center_x: 0.5,
            center_y: 0.5,
            radius: 0.5,
            aspect_ratio: 1.0,
            refractive_index: 1.0,
        };
        Sphere { image: None, ended: true, shader: None, dim }
    }
}

impl Transition for Sphere {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if !self.ended {
            if self.dim.rate <= 0.0 {
                self.ended = true;
            } else {
                self.dim.refractive_index = 2.0 - self.dim.rate;
                self.dim.radius = self.dim.refractive_index / 2.0;
                match &self.image {
                    Some(i) => {
                        graphics::clear(ctx, Color::BLACK);
                        let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

                        if let Some(ref shader) = self.shader {
                            let _lock = graphics::use_shader(ctx, shader);
                            shader.send(ctx, self.dim)?;

                            i.draw(ctx, param)?;
                        }
                    }
                    None => {}
                }
                self.dim.rate -= 0.01;
            }
        }
        Ok(!self.ended)
    }

    fn update_image(&mut self, ctx: &mut Context, image: RgbaImage) {
        self.dim.aspect_ratio = image.width() as f32 / image.height() as f32;
        let shader = graphics::Shader::new(
            ctx,
            "/basic_150.glslv",
            "/sphere_150.glslf",
            self.dim,
            "Dim",
            None,
        ).expect("Error creating shader.");

        self.shader = Some(shader);

        let i = Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &image.into_raw()).unwrap();

        self.image = Some(i);
        self.ended = false;
        self.dim.rate = 1.0;
    }
}