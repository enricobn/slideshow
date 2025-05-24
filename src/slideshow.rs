use std::fmt::Display;
use std::ops::Sub;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use ggez::event::EventHandler;
use ggez::graphics::{Image, ImageFormat, Rect, ScreenImage};
use ggez::*;
use image;
use image::imageops::CatmullRom;
use image::{GenericImage, GenericImageView, ImageBuffer};
use rand::Rng;

use crate::sync_timer::*;
use crate::transitions::distortion::Distortion;
use crate::transitions::fade::Fade;
use crate::transitions::pixels::Pixels;
use crate::transitions::quads::Quads;
use crate::transitions::slides::Slides;
use crate::transitions::sphere::Sphere;
use crate::transitions::transition::{SimpleTransition, Transition};

const LOAD_IMAGE_DELAY: u64 = 5_000; // millis
const UPDATE_DELAY: u64 = 1000 / 60; // millis

pub struct SlideShow {
    timer: SyncTimer,
    file_names: Vec<String>,
    file_index: usize,
    transition: Box<dyn Transition>,
    waiting_for_next_image: bool,
    first: bool,
    image_updated: bool,
    last_time: SystemTime,
    screen_image_buffer: ScreenImage,
}

impl SlideShow {
    pub fn new(args: Vec<String>, screen_image_buffer: ScreenImage) -> SlideShow {
        let folder_name = args.get(1);

        if folder_name.is_none() {
            println!("folder is mandatory",);
            panic!();
        }

        let transition: Box<dyn Transition> = match args.get(2) {
            Some(s) => match s.as_str() {
                "simple" => Box::new(SimpleTransition::new()),
                "pixels" => Box::new(Pixels::new()),
                "quads" => Box::new(Quads::new()),
                "slide" => Box::new(Slides::new(1)),
                "slides" => Box::new(Slides::new(8)),
                "fade" => Box::new(Fade::new()),
                "distortion" => Box::new(Distortion::new()),
                "sphere" => Box::new(Sphere::new()),
                _ => {
                    panic!("Unknown transition {}", s);
                }
            },
            None => Box::new(Fade::new()),
        };

        let directory = Path::new(folder_name.unwrap());

        let paths = directory.read_dir().unwrap();

        let mut file_names = Vec::new();

        for path in paths {
            if let Ok(entry) = path {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let file_name = String::from(entry.path().to_str().unwrap());
                        if file_name.to_uppercase().ends_with("PNG")
                            || file_name.to_uppercase().ends_with("JPG")
                            || file_name.to_uppercase().ends_with("JPEG")
                            || file_name.to_uppercase().ends_with("BMP")
                        {
                            file_names.push(file_name);
                        }
                    }
                }
            }
        }

        if file_names.is_empty() {
            println!("No image files found in {}", folder_name.unwrap());
            panic!();
        }

        let mut rng = rand::thread_rng();

        rng.shuffle(&mut file_names);

        let timer = SyncTimer::new();
        //timer.add(SyncEvent::new("next_image", Duration::from_millis(0), false));
        /*timer.add(SyncEvent::new(
            "draw",
            Duration::from_millis(UPDATE_DELAY),
            true,
        ));

         */

        SlideShow {
            timer,
            file_names,
            file_index: 0,
            transition,
            waiting_for_next_image: true,
            first: true,
            image_updated: false,
            last_time: SystemTime::now(),
            screen_image_buffer,
        }
    }

    fn update_image(&mut self, ctx: &mut Context) -> GameResult<()> {
        let file_name = self.file_names.get(self.file_index).unwrap();
        println!("loading image {}", file_name);

        self.file_index += 1;

        if self.file_index >= self.file_names.len() {
            self.file_index = 0;
        }

        let img = image::open(&file_name).unwrap();

        let rect = Rect {
            x: 0.,
            y: 0.,
            w: ctx.gfx.drawable_size().0,
            h: ctx.gfx.drawable_size().1,
        };

        let width = rect.w;
        let height = rect.h;

        let scale_x = width / img.width() as f32;
        let scale_y = height / img.height() as f32;

        let scale = if scale_x < scale_y { scale_x } else { scale_y };

        let black = image::Rgba { 0: [0, 0, 0, 255] };

        let img = img.resize(
            (img.width() as f32 * scale) as u32,
            (img.height() as f32 * scale) as u32,
            CatmullRom,
        );

        let mut img_rgba = ImageBuffer::from_pixel(width as u32, height as u32, black);

        img_rgba
            .copy_from(
                &img,
                ((width - img.width() as f32) / 2.0) as u32,
                ((height - img.height() as f32) / 2.0) as u32,
            )
            .map_err(|it| GameError::CustomError(it.to_string()))?;

        let image = Image::from_pixels(
            ctx,
            img_rgba.as_raw(),
            ImageFormat::Rgba8UnormSrgb,
            img_rgba.width(),
            img_rgba.height(),
        );

        self.transition.update_image(ctx, image);
        self.waiting_for_next_image = false;

        Ok(())
    }

    fn wait(&mut self, ctx: &mut Context) {
        let frame_time = Duration::from_millis(1_000 / 30);
        let duration = self.last_time.elapsed().unwrap();
        if duration.lt(&frame_time) {
            let duration = frame_time.sub(duration);
            // let start_sleep = SystemTime::now();
            thread::sleep(duration);
            // println!("thread sleep end after {:?}", start_sleep.elapsed());
        }
        self.last_time = SystemTime::now();
    }

    fn get_time_format(&self) -> impl Display {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        //datetime.format("%Y-%m-%d %H:%M:%S%.3f")
        datetime.format("%H:%M:%S%.3f")
    }
}

impl EventHandler<GameError> for SlideShow {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.first || self.timer.fired().iter().any(|it| it == &"next_image") {
            self.update_image(ctx)?;
            self.image_updated = true;
            self.first = false;
        }

        if self.waiting_for_next_image {
            self.wait(ctx);
            return Ok(());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.image_updated {
            ctx.gfx.present(&self.screen_image_buffer.image(ctx))?;
            self.image_updated = false;
            self.wait(ctx);
            return Ok(());
        }

        if self.waiting_for_next_image {
            ctx.gfx.present(&self.screen_image_buffer.image(ctx))?;
            self.wait(ctx);
            return Ok(());
        }

        let mut canvas =
            graphics::Canvas::from_screen_image(ctx, &mut self.screen_image_buffer, None);

        let transaction_finished = !self.transition.draw(ctx, &mut canvas)?;
        canvas.finish(ctx)?;
        ctx.gfx.present(&self.screen_image_buffer.image(ctx))?;

        if transaction_finished && !self.waiting_for_next_image {
            self.timer.add(SyncEvent::new(
                "next_image",
                Duration::from_millis(LOAD_IMAGE_DELAY),
                false,
            ));
            self.waiting_for_next_image = true;
        }
        Ok(())
    }
}
