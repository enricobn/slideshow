use crevice::std140::AsStd140;
use ggez::graphics::{Color, DrawParam, Drawable, Image, ImageFormat};
use ggez::*;
use image::RgbaImage;

use crate::ggez_utils::Point2;
use crate::transitions::transition::Transition;

// Define the input struct for our shader.
#[derive(AsStd140)]
struct Dim {
    rate: f32,
    center_x: f32,
    center_y: f32,
    radius: f32,
    aspect_ratio: f32,
    refractive_index: f32,
}

pub struct Sphere {
    image: Option<RgbaImage>,
    ended: bool,
    shader: Option<graphics::Shader>,
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
        Sphere {
            image: None,
            ended: true,
            shader: None,
            dim,
        }
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
                    Some(image) => {
                        let mut canvas =
                            graphics::Canvas::from_frame(ctx, Some(Color::new(0.0, 0.0, 0.0, 1.0)));
                        let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

                        if let Some(ref shader) = self.shader {
                            let _lock = canvas.set_shader(&shader);
                            let shader_params =
                                graphics::ShaderParamsBuilder::new(&self.dim).build(ctx);
                            canvas.set_shader_params(&shader_params);

                            let i = Image::from_pixels(
                                ctx,
                                image.as_raw(),
                                ImageFormat::Rgba8Uint,
                                image.width(),
                                image.height(),
                            );

                            i.draw(&mut canvas, param);
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
        let shader = graphics::ShaderBuilder::new()
            .fragment_path("/sphere_150.glslf")
            .vertex_path("/basic_150.glslv")
            .build(ctx)
            .unwrap();

        self.shader = Some(shader);

        //let i = Image::from_bytes(ctx, &image.into_raw()).unwrap();

        self.image = Some(image);
        self.ended = false;
        self.dim.rate = 1.0;
    }
}
