use sdl2::pixels::Color;

use crate::helper::Position;

// Window
pub const WINDOW_TITLE: &str = "Flooting";
pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 600;
pub const WINDOW_BACKGROUND: Color = Color::RGB(0, 2, 46);
pub const FPS: u32 = 60;

// Debugging
pub const DEBUG: bool = false;
pub const DEBUG_COLOR: Color = Color::MAGENTA;

// Spaceship
pub const SPACESHIP_WIDTH: u32 = 42;
pub const SPACESHIP_HEIGHT: u32 = 42;
pub const SPACESHIP_LIFE: u8 = 100;
pub const LIFE_COLOR: Color = sdl2::pixels::Color::RGB(0, 255, 21);
pub const INITIAL_SPACESHIP_POSITION: Position = Position {
    x: 40,
    y: ((WINDOW_HEIGHT / 2) - (SPACESHIP_HEIGHT / 2)) as i32,
};

// Missile
pub const MISSILE_WIDTH: u32 = 10;
pub const MISSILE_HEIGHT: u32 = 5;
pub const MISSILE_SPEED: u32 = 10;
pub const SHOTS_PER_SECOND: u16 = 10;
pub const MISSILE_COLOR: Color = Color::YELLOW;

// Pixels
pub const PIXELS_MULTIPLIER_FACTOR: u32 = 12;

// Asteroid
pub const ASTEROIDS_COLOR: Color = Color::RGB(217, 83, 0);
pub const GENERATE_NEW_ASTEROID_AFTER: u32 = 100;
pub const ALLOW_INSIDE_GENERATION: bool = false;
pub const MIN_GENERATED_ASTEROIDS: u8 = 1;
pub const MAX_GENERATED_ASTEROIDS: u8 = 2;
pub const MIN_ASTEROIDS_SPEED: u32 = 2;
pub const MAX_ASTEROIDS_SPEED: u32 = 6;
pub const MIN_ASTEROIDS_SIZE: u8 = 3;
pub const MAX_ASTEROIDS_SIZE: u8 = 6;
pub const ASTEROIDS_ROWS_HEIGHT: u32 = 
    (WINDOW_HEIGHT as u32 / ASTEROIDS_ROWS as u32)
    - (ASTEROIDS_ROWS as u32 * ASTEROIDS_ROWS_MARGIN as u32)
    + ASTEROIDS_ROWS_PADDING;
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
pub const ASTEROIDS_MARGIN: Position = Position {
    x: 100,
    y: (
        WINDOW_HEIGHT as usize - 
        (ASTEROIDS_ROWS * ASTEROIDS_ROWS_HEIGHT as usize)
    ) as i32 / 2,
};