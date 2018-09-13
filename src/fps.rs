use ggez::*;
use ggez::graphics::{Point2, Color};
use ggez::timer::{get_fps};

pub fn draw_fps(ctx: &mut Context, font: &graphics::Font, dest: Point2, color: Color) -> GameResult<()> {
    let fps = get_fps(ctx).round();

    let text = graphics::Text::new(ctx, &format!("fps {}", fps), font)?;

    graphics::draw_ex(
            ctx,
            &text,
            graphics::DrawParam {
                dest: dest,
                color: Some(color),
                ..Default::default()
            },
    )
}