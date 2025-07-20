use raylib::prelude::*;
use crate::config::{GRID_WIDTH, GRID_HEIGHT};

pub fn render(image: &mut Image) {
    let old_data = image.get_image_data();
    let mut new_data = vec![Color::BLACK; (GRID_WIDTH * GRID_HEIGHT) as usize];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let mut neighbors = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && ny >= 0 && nx < GRID_WIDTH && ny < GRID_HEIGHT {
                        let idx = (ny * GRID_WIDTH + nx) as usize;
                        if old_data[idx] == Color::WHITE {
                            neighbors += 1;
                        }
                    }
                }
            }

            let idx = (y * GRID_WIDTH + x) as usize;
            let alive = old_data[idx] == Color::WHITE;

            new_data[idx] = match (alive, neighbors) {
                (true, 2) | (true, 3) => Color::WHITE,
                (false, 3) => Color::WHITE,
                _ => Color::BLACK,
            };
        }
    }

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let idx = (y * GRID_WIDTH + x) as usize;
            image.draw_pixel(x, y, new_data[idx]);
        }
    }
}
