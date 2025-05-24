use crate::ggez_utils::Point2;
use crevice::std140::AsStd140;
use ggez::graphics::{Canvas, DrawParam, Drawable, Image};
use ggez::*;

use crate::transitions::transition::Transition;

// Define the input struct for our shader.
#[derive(AsStd140, Clone)]
pub struct Dim {
    rate: f32,
}

pub struct Distortion {
    image: Option<Image>,
    ended: bool,
    shader: Option<graphics::Shader>,
    dim: Dim,
}

impl Distortion {
    pub fn new() -> Distortion {
        Distortion {
            image: None,
            ended: true,
            shader: None,
            dim: Dim { rate: 1.0 },
        }
    }
}

impl Transition for Distortion {
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<bool> {
        if !self.ended {
            if self.dim.rate <= 0.0 {
                self.ended = true;
            } else {
                match &self.image {
                    Some(image) => {
                        //println!("Distortion 1 {:?}.", SystemTime::now());
                        let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

                        if let Some(ref shader) = self.shader {
                            //println!("Distortion 3 {:?}.", SystemTime::now());

                            let _lock = canvas.set_shader(shader);

                            let shader_params =
                                graphics::ShaderParamsBuilder::new(&self.dim).build(ctx);
                            canvas.set_shader_params(&shader_params);

                            image.draw(canvas, param);
                            //println!("Distortion 6 {:?}.", SystemTime::now());
                        }
                    }
                    None => {}
                }
                self.dim.rate -= 0.01;
            }
        }

        Ok(!self.ended)
    }

    fn update_image(&mut self, ctx: &mut Context, image: Image) {
        let dim = Dim { rate: 0.5 };
        let shader = graphics::ShaderBuilder::new()
            .vertex_path("/simple.vert.wgsl")
            .fragment_path("/distortion.frag.wgsl")
            //.fragment_path("/simple.frag.wgsl")
            .build(ctx)
            .unwrap();
        self.shader = Some(shader);
        self.dim = dim;
        self.image = Some(image);
        self.ended = false;
        self.dim.rate = 1.0;
    }
}
