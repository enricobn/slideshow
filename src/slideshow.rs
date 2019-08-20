use std::path::Path;
use std::time::Duration;

use ggez::*;
use ggez::event::EventHandler;
use ggez::graphics;
use image;
use image::{CatmullRom, ImageBuffer, Rgba};
use rand::Rng;

use sync_timer::*;
use transitions::fade::Fade;
use transitions::pixels::Pixels;
use transitions::quads::Quads;
use transitions::slides::Slides;
use transitions::transition::{SimpleTransition, Transition};
use image::{GenericImage, FilterType};
use image::ColorType::RGBA;

const LOAD_IMAGE_DELAY : u64 = 5_000; // millis

pub struct SlideShow {
    timer: SyncTimer,
    file_names: Vec<String>,
    file_index: usize,
    transition: Box<Transition>,
    waiting: bool,
    first: bool
}

impl SlideShow {

    pub fn new(args: Vec<String>) -> SlideShow {
        let folder_name = args.get(1);

        if folder_name.is_none() {
            println!("folder is mandatory", );
            panic!();
        }

        let transition : Box<Transition> = match args.get(2) {
            Some(s) => {
                match s.as_str() {
                    "simple" => Box::new(SimpleTransition::new()),
                    "pixels" => Box::new(Pixels::new()),
                    "quads" => Box::new(Quads::new()),
                    "slide" => Box::new(Slides::new(1)),
                    "slides" => Box::new(Slides::new(8)),
                    "fade" => Box::new(Fade::new()),
                    _ => {
                        println!("Unknown transition {}", s);
                        panic!();
                    }
                }
            },
            None => Box::new(Fade::new())
        };

        let directory = Path::new(folder_name.unwrap());

        let paths = directory.read_dir().unwrap();

        let mut file_names = Vec::new();

        for path in paths {
            if let Ok(entry) = path {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let file_name = String::from(entry.path().to_str().unwrap());
                        if file_name.to_uppercase().ends_with("PNG") ||
                            file_name.to_uppercase().ends_with("JPG") ||
                            file_name.to_uppercase().ends_with("JPEG") ||
                            file_name.to_uppercase().ends_with("BMP") {
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

        let mut timer = SyncTimer::new();
        timer.add(SyncEvent::new("next_image", Duration::from_millis(0), false));

        SlideShow{timer, file_names, file_index: 0, transition, waiting: true, first: true}
    }

    fn update_image(&mut self, ctx: &mut Context) -> GameResult<()> {
        let file_name = self.file_names.get(self.file_index).unwrap();
        println!("loading image {}", file_name);

        self.file_index += 1;

        if self.file_index >= self.file_names.len() {
            self.file_index = 0;
        }

        let mut img = image::open(&file_name).unwrap();

        let rect = graphics::screen_coordinates(ctx);

        let width = rect.w;
        let height = rect.h;

        let scale_x = width / img.width() as f32;
        let scale_y = height / img.height() as f32;

        let scale = if scale_x < scale_y { scale_x } else {scale_y};

        let img = img.resize((img.width() as f32 * scale) as u32,
                             (img.height() as f32 * scale) as u32, CatmullRom);

        let black = Rgba{ data: [0, 0, 0, 255] };

        let mut img_rgba = ImageBuffer::from_pixel(width as u32, height as u32, black);

        img_rgba.copy_from(&img, ((width - img.width() as f32) / 2.0) as u32, ((height - img.height() as f32) / 2.0) as u32);

        self.transition.update_image(ctx, img_rgba);
        self.waiting = false;

        Ok(())
    }

}

impl EventHandler for SlideShow {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fired = self.timer.fired().clone();

        for id in fired {
            if id == "next_image" {
                self.update_image(ctx)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // TODO I don't know why, but I have to issue an empty present otherwise in the first draw
        // something weird happens with the image coordinates!
        if self.first {
            self.first = false;
            graphics::present(ctx);
            return Ok(());
        }

        let running = self.transition.draw(ctx)?;

        graphics::present(ctx);

        if !running && !self.waiting {
            self.timer.add(SyncEvent::new("next_image", Duration::from_millis(LOAD_IMAGE_DELAY), false));
            self.waiting = true;
        }

        timer::yield_now();

        Ok(())
    }

}
