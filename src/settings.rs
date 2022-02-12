use sdl2::pixels::Color;

use crate::helper::Position;

pub const WINDOW_TITLE: &str = "Flooting";
pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 400;
pub const WINDOW_BACKGROUND: Color = Color::RGB(0, 0, 0);

pub const FPS: u32 = 60;

pub const SPACESHIP_WIDTH: u32 = 42;
pub const SPACESHIP_HEIGHT: u32 = 42;

pub const INITIAL_SPACESHIP_POSITION: Position = Position {
    x: 40,
    y: ((WINDOW_HEIGHT / 2) - (SPACESHIP_HEIGHT / 2)) as i32,
};

pub const ASTEROIDS_MARGIN: Position = Position {
    x: 100,
    y: 30,
};

pub const GENERATE_NEW_ASTEROID_AFTER: u32 = 300;

pub const ALLOW_INSIDE_GENERATION: bool = false;
// pub const ALLOW_INSIDE_GENERATION: bool = true;

// pub const MIN_GENERATED_ASTEROIDS: u8 = 1;
// pub const MAX_GENERATED_ASTEROIDS: u8 = 3;
pub const MIN_GENERATED_ASTEROIDS: u8 = 250;
pub const MAX_GENERATED_ASTEROIDS: u8 = 255;

pub const MIN_ASTEROIDS_SPEED: u32 = 20;
pub const MAX_ASTEROIDS_SPEED: u32 = 30;
// pub const MIN_ASTEROIDS_SPEED: u32 = 2;
// pub const MAX_ASTEROIDS_SPEED: u32 = 6;

pub const MIN_ASTEROIDS_SIZE: u8 = 3;
pub const MAX_ASTEROIDS_SIZE: u8 = 7;

pub const SHOTS_PER_SECOND: u16 = 3;

pub const PIXELS_MULTIPLIER_FACTOR: u32 = 12;