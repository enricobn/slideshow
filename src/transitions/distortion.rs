use crevice::std140::AsStd140;
use ggez::graphics::ShaderParams;
use ggez::*;
use image::RgbaImage;

use crate::transitions::transition::Transition;

// Define the input struct for our shader.
#[derive(AsStd140, Clone)]
pub struct Dim {
    rate: f32,
}

pub struct Distortion {
    image: Option<RgbaImage>,
    ended: bool,
    shader: Option<graphics::Shader>,
    dim: Dim,
    params: Option<ShaderParams<Dim>>,
}

impl Distortion {
    pub fn new() -> Distortion {
        Distortion {
            image: None,
            ended: true,
            shader: None,
            dim: Dim { rate: 1.0 },
            params: None,
        }
    }
}

impl Transition for Distortion {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<bool> {
        /*
        if !self.ended {
            if self.dim.rate <= 0.0 {
                self.ended = true;
            } else {
                match &self.image {
                    Some(i) => {
                        //println!("Distortion 1 {:?}.", SystemTime::now());
                        let mut canvas = graphics::Canvas::from_frame(
                            ctx,
                            Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                        );
                        let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

                        if let Some(ref shader) = self.shader {
                            if let Some(ref params) = self.params {
                                //println!("Distortion 3 {:?}.", SystemTime::now());

                                let _lock = canvas.set_shader(shader);

                                canvas.set_shader_params(&params);

                                i.draw(&mut canvas, param);
                                //println!("Distortion 6 {:?}.", SystemTime::now());
                                canvas.finish(ctx)?;
                            }
                        }
                    }
                    None => {}
                }
                self.dim.rate -= 0.01;
            }
        }

         */
        Ok(!self.ended)
    }

    fn update_image(&mut self, ctx: &mut Context, image: RgbaImage) {
        let dim = Dim { rate: 0.5 };
        let shader = graphics::ShaderBuilder::new()
            .fragment_path("/distortion_150.wgsl")
            .vertex_path("/basic_150.wgsl")
            .build(ctx)
            .unwrap();
        self.shader = Some(shader);
        self.dim = dim;
        self.params = Some(graphics::ShaderParamsBuilder::new(&self.dim).build(ctx));
        self.image = Some(image);
        self.ended = false;
        self.dim.rate = 1.0;
    }
}
