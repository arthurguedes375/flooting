extern crate sdl2;

pub mod time;
pub mod settings;
pub mod helper;
pub mod rectangle;
pub mod game;
pub mod ui;

use ui::UiSettings;
use game::Game;

fn main() {
    let mut game = Game::new(UiSettings {
        title: String::from(settings::WINDOW_TITLE),
        width: settings::WINDOW_WIDTH,
        height: settings::WINDOW_HEIGHT,
    });

    game.init();
}
