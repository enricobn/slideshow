use std::path::Path;
use std::time::{Instant, Duration, SystemTime};

use ggez::*;
use ggez::event::EventHandler;
use ggez::graphics;
use image;
use image::{CatmullRom, ImageBuffer, Rgba};
use image::GenericImage;
use rand::Rng;

use crate::sync_timer::*;
use crate::transitions::distortion::Distortion;
use crate::transitions::fade::Fade;
use crate::transitions::pixels::Pixels;
use crate::transitions::quads::Quads;
use crate::transitions::slides::Slides;
use crate::transitions::sphere::Sphere;
use crate::transitions::transition::{SimpleTransition, Transition};
use crate::utils::format_duration;

const LOAD_IMAGE_DELAY: u64 = 5_000; // millis
const UPDATE_DELAY: u64 = 16; // millis

pub struct SlideShow {
    timer: SyncTimer,
    file_names: Vec<String>,
    file_index: usize,
    transition: Box<dyn Transition>,
    waiting_for_next_image: bool,
    first: bool,
    fired_events: Vec<&'static str>,
    image_updated: bool,
}

impl SlideShow {
    pub fn new(args: Vec<String>) -> SlideShow {
        let folder_name = args.get(1);

        if folder_name.is_none() {
            println!("folder is mandatory", );
            panic!();
        }

        let transition: Box<dyn Transition> = match args.get(2) {
            Some(s) => {
                match s.as_str() {
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
                }
            }
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
        //timer.add(SyncEvent::new("next_image", Duration::from_millis(0), false));
        timer.add(SyncEvent::new("draw", Duration::from_millis(UPDATE_DELAY), true));

        SlideShow { timer, file_names, file_index: 0, transition, waiting_for_next_image: true, first: true ,
        fired_events: Vec::new(), image_updated: false}
    }

    fn update_image(&mut self, ctx: &mut Context) -> GameResult<()> {
        let file_name = self.file_names.get(self.file_index).unwrap();
        println!("loading image {}", file_name);

        self.file_index += 1;

        if self.file_index >= self.file_names.len() {
            self.file_index = 0;
        }

        let img = image::open(&file_name).unwrap();

        let rect = graphics::screen_coordinates(ctx);

        let width = rect.w;
        let height = rect.h;

        let scale_x = width / img.width() as f32;
        let scale_y = height / img.height() as f32;

        let scale = if scale_x < scale_y { scale_x } else { scale_y };

        let img = img.resize((img.width() as f32 * scale) as u32,
                             (img.height() as f32 * scale) as u32, CatmullRom);

        let black = Rgba { data: [0, 0, 0, 255] };

        let mut img_rgba = ImageBuffer::from_pixel(width as u32, height as u32, black);

        img_rgba.copy_from(&img, ((width - img.width() as f32) / 2.0) as u32, ((height - img.height() as f32) / 2.0) as u32);

        self.transition.update_image(ctx, img_rgba);
        self.waiting_for_next_image = false;

        Ok(())
    }

}

impl EventHandler<ggez::GameError> for SlideShow {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        //println!("update start {:?}.", SystemTime::now());

        self.fired_events = self.timer.fired().clone();

        if self.first || self.fired_events.iter().any(|it| it == &"next_image") {
            self.update_image(ctx)?;
            self.image_updated = true;
            self.first = false;
        }

        //println!("update end {:?}.", SystemTime::now());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //println!("draw start {:?}.", SystemTime::now());

        if self.image_updated {
            //println!("draw: image_updated");
            self.image_updated = false;
            //graphics::present(ctx);
            return Ok(());
        }

        if !self.fired_events.iter().any(|it| it == &"draw") {
            //graphics::present(ctx);
            return Ok(());
        }

        //let start = Instant::now();
        let transaction_finished = !self.transition.draw(ctx)?;
        //println!("draw: transition done: {}.", transaction_finished);

        //println!("draw 1 {:?}.", SystemTime::now());

        if transaction_finished && !self.waiting_for_next_image {
            //println!("draw: start waiting for next_image");
            self.timer.add(SyncEvent::new("next_image", Duration::from_millis(LOAD_IMAGE_DELAY), false));
            self.waiting_for_next_image = true;
        } else if !self.waiting_for_next_image {
            //println!("draw 2 {:?}.", SystemTime::now());

            //println!("draw: main present");

            graphics::present(ctx);

            //println!("draw 3 {:?}.", SystemTime::now());


            //println!("draw end {:?}.", SystemTime::now());
        }

        //timer::yield_now();

        Ok(())
    }

}
