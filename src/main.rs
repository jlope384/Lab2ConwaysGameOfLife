mod game;
mod primes;
mod config;

use raylib::prelude::*;
use crate::game::render;
use crate::primes::initialize_grid_with_primes;
use crate::config::{GRID_WIDTH, GRID_HEIGHT, CELL_SCALE};

fn main() {
    let (screen_width, screen_height) = (GRID_WIDTH * CELL_SCALE, GRID_HEIGHT * CELL_SCALE);
    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Conway's Game of Life - Primes")
        .build();

    rl.set_target_fps(10);

    let mut image = Image::gen_image_color(GRID_WIDTH, GRID_HEIGHT, Color::BLACK);
    initialize_grid_with_primes(&mut image);

    while !rl.window_should_close() {
        render(&mut image);
        let texture = rl.load_texture_from_image(&thread, &image).unwrap();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(
            &texture,
            Vector2::new(0.0, 0.0),
            0.0,
            screen_width as f32 / texture.width() as f32,
            Color::WHITE,
        );
    }
}
