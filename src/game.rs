use std::time::{SystemTime, UNIX_EPOCH};

use crate::resource::GameWorld;
use pixels::Pixels;

const COLOR: [u8; 4] = [255, 255, 255, 255];
const SIZE: u32 = 20;

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
