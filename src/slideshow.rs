use std;
use std::time::Duration;
use std::path::Path;

use ggez::*;
use ggez::event::{EventHandler};
use transition::*;
use pixels::*;
use quads::*;
use slides::*;

use image;

use sync_timer::*;

const LOAD_IMAGE_DELAY : u64 = 5_000; // millis

pub struct SlideShow {
    timer: SyncTimer,
    file_names: Vec<String>,
    file_index: usize,
    transition: Box<Transition>,
    waiting: bool
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
                    "slides" => Box::new(Slides::new()),
                    _ => {
                        println!("Unknown transition {}", s);
                        panic!();
                    }
                }
            },
            None => Box::new(Slides::new())
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

        let mut timer = SyncTimer::new();
        timer.add(SyncEvent::new("next_image", Duration::from_millis(0), false));

        SlideShow{timer: timer, file_names: file_names, file_index: 0,
            transition: transition, waiting: true}
    }

    fn update_image(&mut self, ctx: &mut Context) -> GameResult<()> {
        let file_name = self.file_names.get(self.file_index).unwrap();
        println!("loading image {}", file_name);

        self.file_index += 1;

        if self.file_index >= self.file_names.len() {
            self.file_index = 0;
        }

        let img = image::open(&file_name).unwrap();
        let img_rgba = img.to_rgba();

        self.transition.update(ctx, img_rgba);
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
        let running = self.transition.draw(ctx)?;

        graphics::present(ctx);

        if !running && !self.waiting {
            self.timer.add(SyncEvent::new("next_image", Duration::from_millis(LOAD_IMAGE_DELAY), false));
            self.waiting = true;
        }

        Ok(())
    }

}
