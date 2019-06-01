use std;
use std::time::Duration;
use std::path::Path;

use ggez::*;
use ggez::event::{EventHandler};
use ggez::graphics::{Image};
use ggez::timer::{get_delta, duration_to_f64};
use transition::*;
use pixels::*;

use image;

use sync_timer::*;

const LOAD_IMAGE_DELAY : u64 = 20_000; // millis

pub struct SlideShow {
    timer: SyncTimer,
    file_names: Vec<String>,
    file_index: usize,
    transition: Box<Transition>,
    first: bool
}

impl SlideShow {

    pub fn new(args: Vec<String>) -> SlideShow {
        let folder_name = args.get(1);

        if folder_name.is_none() {
            println!("folder is mandatory", );
            panic!();
        }

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
        timer.add(SyncEvent::new("next_image", Duration::from_millis(LOAD_IMAGE_DELAY), true));

        let mut s = SlideShow{timer: timer, file_names: file_names, file_index: 0,
            transition: Box::new(Pixels::new()), first: true};

        &s.update_image();
        return s;
    }

    fn update_image(&mut self) -> GameResult<()> {
        let file_name = self.file_names.get(self.file_index).unwrap();
        println!("loading image {}", file_name);

        self.file_index += 1;

        if self.file_index >= self.file_names.len() {
            self.file_index = 0;
        }

        let img = image::open(&file_name).unwrap();
        let img_rgba = img.to_rgba();

        self.transition.update(img_rgba);
        Ok(())
    }


}

impl EventHandler for SlideShow {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta_duration = get_delta(ctx);
        let delta = duration_to_f64(delta_duration);

        let fired = self.timer.fired().clone();

        for id in fired {
            if id == "next_image" {
                self.update_image()?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // graphics::clear(ctx);

        let redraw = self.transition.draw(ctx)?;

        // draw_fps(ctx, &self.font, graphics::Point2::new(10.0, 10.0), graphics::Color::from((255, 255, 255, 255)))?;

        // if Instant::now().duration_since(self.last_fps_print).as_secs() >= 1 {
        //     let fps = get_fps(ctx).round();
        //     println!("fps: {}", fps);
        //     self.last_fps_print = Instant::now();
        // }

        if redraw {
            graphics::present(ctx);
        }

        Ok(())
    }

}
