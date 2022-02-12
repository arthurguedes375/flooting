use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::Window;
use sdl2::render::Canvas;

use sdl2::rect::Rect;

use crate::settings;
use crate::rectangle::{Rectangle, Size};
use crate::game;

use game::{Game, Debug};

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
        let mut asteroids_rects: Vec<Rect> = vec![];

        for row in game.asteroids.iter() {
            for asteroid in row.iter() {
                let size = Ui::to_pixels(asteroid.size as u32);
                let corners = Rectangle {
                    position: asteroid.position,
                    size: Size::Square(size),
                }.get_corners();
                if corners.top_left.x > settings::WINDOW_WIDTH as i32 { continue; }
                asteroids_rects.push(Rect::new(corners.top_left.x, corners.top_left.y, size, size));
            }
        };

        game.ui.canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 6, 0));
        game.ui.canvas.fill_rects(&asteroids_rects).unwrap();
    }

    pub fn draw_spaceship(game: &mut Game) {
        if game.spaceship.shooting {
            game.ui.canvas.set_draw_color(sdl2::pixels::Color::from((255, 0, 68)));
        } else {
            game.ui.canvas.set_draw_color(sdl2::pixels::Color::from((38, 38, 255)));
        }
        game.ui.canvas.fill_rect(Rect::new(
            game.spaceship.position.x, 
            game.spaceship.position.y,
            settings::SPACESHIP_WIDTH,
            settings::SPACESHIP_HEIGHT,
        )).unwrap();
    }

    pub fn debug(game: &mut Game) {

        if game.debug_options.generation_line {
            // Draw Generation Line
            game.ui.canvas.set_draw_color(settings::DEBUG_COLOR);
            game.ui.canvas.fill_rect(
                Rect::new(
                    settings::WINDOW_WIDTH as i32
                    - settings::GENERATE_NEW_ASTEROID_AFTER as i32,
                    0,
                    1,
                    settings::WINDOW_HEIGHT
                )
            ).unwrap();
        }

        if game.debug_options.rows_starting_line {
            let mut rects: Vec<Rect> = vec![];
            for row in 0..game.asteroids.len() {
                let y_position = Game::get_row_y_position(row);
                rects.push(Rect::new(
                    0,
                    y_position as i32,
                    settings::WINDOW_WIDTH,
                    1
                ));
            }

            game.ui.canvas.set_draw_color(settings::DEBUG_COLOR);
            game.ui.canvas.fill_rects(&rects).unwrap();
        }


    }

    pub fn draw(game: &mut Game) {
        game.ui.canvas.set_draw_color(settings::WINDOW_BACKGROUND);
        game.ui.canvas.clear();

        match game.debug {
            Debug::Debugging => {
                Ui::debug(game);
            }
            _ => {}
        }

        Ui::draw_spaceship(game);
        Ui::draw_asteroids(game);

        game.ui.canvas.present();
    }
}