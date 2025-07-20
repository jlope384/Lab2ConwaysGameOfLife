use raylib::prelude::*;
use crate::config::{GRID_WIDTH, GRID_HEIGHT};

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn initialize_grid_with_primes(image: &mut Image) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let idx = (y * GRID_WIDTH + x) as usize;
            if is_prime(idx) {
                image.draw_pixel(x, y, Color::WHITE);
            } else {
                image.draw_pixel(x, y, Color::BLACK);
            }
        }
    }
}
