use std::time::{SystemTime, UNIX_EPOCH};

use crate::{constants::POINT_MATRIX, resource::GameWorld};
use pixels::Pixels;

const COLOR: [u8; 4] = [255, 255, 255, 255];
const SIZE: u32 = 15;

pub fn init(width: u32, height: u32) -> GameWorld {
    GameWorld::new(width, height, SIZE)
}

pub fn draw_object(points: &Vec<[u32; 2]>, pixels: &mut Pixels<'static>, width: u32) {
    for point in points {
        let frame = pixels.frame_mut();
        draw_on_point(point[0], point[1], COLOR, frame, width);
    }
}

/// Draws a single pixel at (x, y) with alpha blending
fn draw_on_point(x: u32, y: u32, color: [u8; 4], frame: &mut [u8], width: u32) {
    let i = ((y * width + x) * 4) as usize;

    if i + 3 < frame.len() {
        let alpha = color[3] as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;

        frame[i] = (color[0] as f32 * alpha + frame[i] as f32 * inv_alpha) as u8;
        frame[i + 1] = (color[1] as f32 * alpha + frame[i + 1] as f32 * inv_alpha) as u8;
        frame[i + 2] = (color[2] as f32 * alpha + frame[i + 2] as f32 * inv_alpha) as u8;
        frame[i + 3] = 255;
    }
}

pub fn random(limit: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos % limit
}

// Numbers
pub fn draw_number(
    number: u32,
    x: u32,
    y: u32,
    pixels: &mut Pixels<'static>,
    width: u32,
    game_world: &mut GameWorld,
) {
    for (current_char, char) in number.to_string().chars().enumerate() {
        let idx = char.to_digit(10).unwrap() as usize;
        let matrix = POINT_MATRIX[idx];
        for (i, row) in matrix.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 1 {
                    let point_x = (x + j as u32) + (5 * current_char as u32) + 1;
                    let point_y = y + i as u32;
                    let square = game_world.add_square(point_x, point_y);
                    draw_object(&square.points, pixels, width);
                }
            }
        }
    }
}
