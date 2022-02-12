use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::Window;
use sdl2::render::Canvas;

use sdl2::rect::Rect;

use crate::settings;
use crate::rectangle::{Rectangle, Size};
use crate::game;

use game::{Game};

pub struct UiSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Ui {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Ui {
    pub fn to_pixels(value: u32) -> u32 {
        return value * settings::PIXELS_MULTIPLIER_FACTOR;
    }

    pub fn draw_asteroids(game: &mut Game) {
        let asteroids_rects = game.asteroids.iter().map(|asteroid| {
            let size = Ui::to_pixels(asteroid.size as u32);
            let corners = Rectangle {
                position: asteroid.position,
                size: Size::Square(size),
            }.get_corners();
            Rect::new(corners.top_left.x, corners.top_left.y, size, size)
        }).collect::<Vec<Rect>>();
        
        game.ui.canvas.set_draw_color(sdl2::pixels::Color::CYAN);
        game.ui.canvas.draw_rects(&asteroids_rects[..]).unwrap();
    }

    pub fn draw(game: &mut Game) {
        println!("{}", game.asteroids.len());
        game.ui.canvas.set_draw_color(settings::WINDOW_BACKGROUND);
        game.ui.canvas.clear();

        game.ui.canvas.set_draw_color(sdl2::pixels::Color::MAGENTA);
        game.ui.canvas.draw_rect(Rect::new(settings::WINDOW_WIDTH as i32 - settings::GENERATE_NEW_ASTEROID_AFTER as i32, 0, 10, settings::WINDOW_HEIGHT)).unwrap();

        Ui::draw_asteroids(game);

        game.ui.canvas.present();
    }
}