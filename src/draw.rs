use piston_window::{ rectangle, Context, G2d };
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    return (game_coord as f64) * BLOCK_SIZE;
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y: f64 = to_coord(y);

    rectangle(
        color,
        [ gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE ],
        context.transform,
        g
    );
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, context: &Context, g: &mut G2d) {
    let x: f64 = to_coord(x);
    let y: f64 = to_coord(y);

    rectangle(
        color,
        [ x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64) ],
        context.transform,
        g
    );
}

