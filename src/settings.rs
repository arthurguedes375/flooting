use sdl2::pixels::Color;

use crate::physics::Position;

use crate::time;
use crate::game::{DebugOptions};
use crate::rectangle::{Rectangle, Size, RectangleSize};

// Window
pub const WINDOW_TITLE: &str = "Flooting";
pub const WINDOW_WIDTH: u32 = 1200;
// pub const WINDOW_HEIGHT: u32 = 600;
pub const WINDOW_HEIGHT: u32 = 845;
pub const FPS: u32 = 60;

// NextGen
pub const NEXT_GENERATION_DELAY: u128 = time::to_nano(2000);

// Sprites
pub const SPRITES_FILE_PATH: &str = "./assets/sprites/sprite.png";
pub const ASTEROID_SPRITE_RECTANGLE: Rectangle = Rectangle {
    position: Position {
        x: 0,
        y: 0,
    },
    size: Size::Rectangle(RectangleSize {
        width: 285,
        height: 246,
    })
};
pub const SPACESHIP_SPRITE_RECTANGLE: Rectangle = Rectangle {
    position: Position {
        x: 0,
        y: 250,
    },
    size: Size::Rectangle(RectangleSize {
        width: 241,
        height: 209,
    })
};
pub const MISSILE_SPRITE_RECTANGLE: Rectangle = Rectangle {
    position: Position {
        x: 0,
        y: 460,
    },
    size: Size::Rectangle(RectangleSize {
        width: 254,
        height: 67,
    })
};
pub const BACKGROUND_SPRITE_RECTANGLE: Rectangle = Rectangle {
    position: Position {
        x: 287,
        y: 0,
    },
    size: Size::Rectangle(RectangleSize {
        width: 852,
        height: 480,
    }),
};

// Debugging
pub const DEBUG_FONT_PATH: &str = "./assets/fonts/debug.ttf";
pub const DEBUG_FONT_POINT_SIZE: u16 = 15;
pub const DEBUG: bool = false;
pub const DEBUG_COLOR: Color = Color::MAGENTA;
pub const DEFAULT_DEBUG_OPTIONS: DebugOptions = DebugOptions {
    generation_line: true,
    rows: true,
    game_state: true,
    invincible: true,
    asteroid_generation: true,
};

// Spaceship
pub const SPACESHIP_WIDTH: u32 = 32;
pub const SPACESHIP_HEIGHT: u32 = 42;
pub const SPACESHIP_LIFE: u8 = 100;
pub const LIFE_COLOR: Color = sdl2::pixels::Color::RGB(0, 255, 21);
pub const INITIAL_SPACESHIP_POSITION: Position = Position {
    x: 40,
    y: ((WINDOW_HEIGHT / 2) - (SPACESHIP_HEIGHT / 2)) as i32,
};

// Missile
pub const MISSILE_WIDTH: u32 = 15;
pub const MISSILE_HEIGHT: u32 = 10;
pub const SHOTS_PER_SECOND: u16 = 10;
pub const MISSILE_COLOR: Color = Color::YELLOW;

// Pixels
pub const PIXELS_MULTIPLIER_FACTOR: u32 = 22;

// Asteroid
pub const GENERATE_NEW_ASTEROID_AFTER: u32 = 100;
pub const ALLOW_INSIDE_GENERATION: bool = false;
pub const MIN_GENERATED_ASTEROIDS: u8 = 1;
pub const MAX_GENERATED_ASTEROIDS: u8 = 1;
// pub const MIN_GENERATED_ASTEROIDS: u8 = 199;
// pub const MAX_GENERATED_ASTEROIDS: u8 = 200;
pub const MIN_ASTEROIDS_SPEED: u32 = 2;
pub const MAX_ASTEROIDS_SPEED: u32 = 3;
pub const MIN_ASTEROIDS_SIZE: u8 = 2;
pub const MAX_ASTEROIDS_SIZE: u8 = 3;
pub const ASTEROIDS_ROWS_HEIGHT: u32 = WINDOW_HEIGHT as u32 / ASTEROIDS_ROWS as u32;

pub const ASTEROIDS_ROWS_PADDING: u32 = 25;
pub const ASTEROIDS_ROWS: usize = (
    WINDOW_HEIGHT /
    (
        MAX_ASTEROIDS_SIZE as u32 * PIXELS_MULTIPLIER_FACTOR
        + ASTEROIDS_ROWS_PADDING
    )
) as usize;
pub const ASTEROIDS_MARGIN: Position = Position {
    x: 100,
    y: 0,
};