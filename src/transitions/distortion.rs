use gfx::*;
use ggez::*;
use ggez::graphics::{BLACK, Drawable, DrawParam, Image};
use image::RgbaImage;

use ggez_utils::Point2;
use transitions::transition::Transition;

// Define the input struct for our shader.
gfx_defines! {
    constant Dim {
        rate: f32 = "u_Rate",
    }
}

pub struct Distortion {
    image: Option<Image>,
    ended: bool,
    shader: Option<graphics::Shader<Dim>>,
    dim: Dim,
}

impl Distortion {
    pub fn new() -> Distortion {
        Distortion { image: None, ended: true, shader: None, dim: Dim { rate: 1.0 } }
    }
}

impl Transition for Distortion {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        if !self.ended {
            if self.dim.rate <= 0.0 {
                self.ended = true;
            } else {
                match &self.image {
                    Some(i) => {
                        graphics::clear(ctx, BLACK);
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
        let dim = Dim { rate: 0.5 };
        let shader = graphics::Shader::new(
            ctx,
            "/basic_150.glslv",
            "/distortion_150.glslf",
            dim,
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