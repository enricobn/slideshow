use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, Rect};
use ggez::*;

pub type Point2 = nalgebra::Point2<f32>;

pub fn draw_rect(
    ctx: &mut Context,
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: &Color,
    mode: DrawMode,
) -> GameResult {
    let rect = Rect::new(0.0, 0.0, width, height);

    let mesh = Mesh::new_rectangle(ctx, mode, rect, *color)?;

    let param = DrawParam::new().dest(Point2::new(x, y));

    mesh.draw(canvas, param);
    Ok(())
}

/*
pub fn draw_fps(ctx: &mut Context, world: &World, font: graphics::Font, color: Color) -> GameResult<()> {
    let fps = fps(ctx).round() as i32;

    let mut text = graphics::Text::new(&*format!("fps {}", fps));
    text.set_font(font, Scale::uniform(32.0));

    let dest = Point2::new(5.0, world.height() as f32 - text.height(ctx) as f32 - 5.0);

    graphics::draw(
            ctx,
            &text,
            (dest,color,)
    )

}

pub fn center_text(ctx: &mut Context, world: &World, text: &Text, color: Color) -> GameResult<()> {
    let x = (world.width() -text.width(ctx) as f32) / 2.0;
    let y = (world.height() - text.height(ctx) as f32) / 2.0;
    draw(ctx, text, (Point2::new(x, y), color,))
}

pub fn center_text_h(ctx: &mut Context, world: &World, text: &Text, y: f32, color: Color) -> GameResult<()> {
    let x = (world.width() -text.width(ctx) as f32) / 2.0;
    draw(ctx, text, (Point2::new(x, y), color,))
}
*/
