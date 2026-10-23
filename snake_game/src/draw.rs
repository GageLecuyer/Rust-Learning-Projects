use piston_window::graphics::{rectangle, Context, Graphics};

const BLOCK_SIZE: f64 = 25.0;

pub fn draw_block<G: Graphics>(
    color: [f32; 4],
    x: i32,
    y: i32,
    con: &Context,
    g: &mut G,
) {
    rectangle(
        color,
        [
            x as f64 * BLOCK_SIZE + 1.0,
            y as f64 * BLOCK_SIZE + 1.0,
            BLOCK_SIZE - 2.0,
            BLOCK_SIZE - 2.0,
        ],
        con.transform,
        g,
    );
}