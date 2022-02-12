use sdl2::pixels::Color;

use crate::helper::Position;

pub const WINDOW_TITLE: &str = "Flooting";
pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 600;
pub const WINDOW_BACKGROUND: Color = Color::RGB(41, 13, 1);

// Debugging
pub const DEBUG: bool = false;
pub const DEBUG_COLOR: Color = Color::MAGENTA;

pub const FPS: u32 = 60;

pub const SPACESHIP_WIDTH: u32 = 42;
pub const SPACESHIP_HEIGHT: u32 = 42;

pub const INITIAL_SPACESHIP_POSITION: Position = Position {
    x: 40,
    y: ((WINDOW_HEIGHT / 2) - (SPACESHIP_HEIGHT / 2)) as i32,
};

pub const ASTEROIDS_MARGIN: Position = Position {
    x: 100,
    y: (
        WINDOW_HEIGHT as usize - 
        (ASTEROIDS_ROWS * ASTEROIDS_ROWS_HEIGHT as usize)
    ) as i32 / 2,
};

pub const GENERATE_NEW_ASTEROID_AFTER: u32 = 300;

pub const ALLOW_INSIDE_GENERATION: bool = false;

pub const MIN_GENERATED_ASTEROIDS: u8 = 1;
pub const MAX_GENERATED_ASTEROIDS: u8 = 3;

pub const MIN_ASTEROIDS_SPEED: u32 = 2;
pub const MAX_ASTEROIDS_SPEED: u32 = 4;

pub const MIN_ASTEROIDS_SIZE: u8 = 3;
pub const MAX_ASTEROIDS_SIZE: u8 = 5;

pub const SHOTS_PER_SECOND: u16 = 3;

pub const PIXELS_MULTIPLIER_FACTOR: u32 = 12;

pub const ASTEROIDS_ROWS_HEIGHT: u32 = 
    (WINDOW_HEIGHT as u32 / ASTEROIDS_ROWS as u32)
    - (ASTEROIDS_ROWS as u32 * ASTEROIDS_ROWS_MARGIN as u32)
    + ASTEROIDS_ROWS_PADDING
;
pub const ASTEROIDS_ROWS_MARGIN: u32 = 5;
pub const ASTEROIDS_ROWS_PADDING: u32 = 25;
pub const ASTEROIDS_ROWS: usize = (
    WINDOW_HEIGHT /
    (
        MAX_ASTEROIDS_SIZE as u32 * PIXELS_MULTIPLIER_FACTOR
        + ASTEROIDS_ROWS_PADDING
        + ASTEROIDS_ROWS_MARGIN
    )
) as usize;