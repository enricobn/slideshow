extern crate ggez;
extern crate rand;
use ggez::*;
use ggez::conf::FullscreenType;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics::{DrawMode,Point2,Rect,Color};
use std::ops::IndexMut;
use rand::*;
use std::time::Instant;

fn main() {
    let mut c = conf::Conf::new();
    c.window_setup = c.window_setup.title("Quad fight");
    // c.window_mode.fullscreen_type = FullscreenType::Desktop;
    // c.window_mode.width = 60;
    // c.window_mode.height = 60;
    c.window_mode.vsync = true;

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    for (width, height) in ggez::graphics::get_fullscreen_modes(ctx, 0).unwrap() {
        println!("{}x{}", width, height);
    }

    let mut state = MainState::new();

    event::run(ctx, &mut state).unwrap();
}

struct Enemy {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    vx: f32,
    vy:f32,
}

impl Enemy {

    fn new(x: f32, y: f32, width: f32, height: f32, color: Color, vx: f32, vy:f32,) -> Enemy {
        Enemy{x: x, y: y, width: width, height: height, color: color, vx: vx, vy: vy}
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.y < 0.0 {
            self.y = -self.y;
            self.vy = -self.vy;
        }

        if self.y + self.height > 600.0 {
            self.y = 600.0 - self.height;//self.y - (self.y + self.height - 600.0);
            self.vy = -self.vy;
        }
    }

}

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Player {

    fn new() -> Player {
        Player{x: 10.0, y: 300.0, width: 50.0, height: 20.0, color: Color::new(1.0, 1.0, 1.0, 1.0)}
    }

}

struct MainState {
    player: Player,
    enemies: Vec<Enemy>,
    last_enemy_added: Instant,
}

impl MainState {

    fn new() -> MainState {
        MainState{player: Player::new(), enemies: Vec::new(), last_enemy_added: Instant::now()}
    }

    fn last_enemy_added_millis(&self) -> u64 {
        let duration = self.last_enemy_added.elapsed();

        duration.as_secs() * 1_000 + duration.subsec_millis() as u64
    }

}

impl event::EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut rng = thread_rng();

        if self.enemies.len() < 10 &&  self.last_enemy_added_millis() > 300 {
            let vy = rng.gen_range(-10.0, 10.0);
            self.enemies.push(Enemy::new(800.0, rng.gen_range(0.0, 500.0), 100.0, 100.0, Color::new(1.0, 0.0, 0.0, 1.0), -10.0, vy));
            self.last_enemy_added = Instant::now();
        }

        let mut to_remove = Vec::new();
        let mut i: usize = 0;
        while i < self.enemies.len() {
            let enemy = self.enemies.index_mut(i);
            enemy.update();
            if enemy.x + enemy.width < 0.0 {
                to_remove.push(i);
            }
            i += 1;
        }

        for i in to_remove {
            self.enemies.remove(i);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, self.player.color)?;
        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                    self.player.x, 
                    self.player.y, 
                    self.player.width, 
                    self.player.height
                ))?;

        for enemy in &self.enemies {
            graphics::set_color(ctx, enemy.color)?;
            graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                        enemy.x, 
                        enemy.y, 
                        enemy.width, 
                        enemy.height
                    ))?;
        }

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

}
