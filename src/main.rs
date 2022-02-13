extern crate sdl2;

pub mod time;
pub mod settings;
pub mod helper;
pub mod rectangle;
pub mod game;
pub mod ui;

use  std::sync::mpsc;

use ui::UiSettings;
use game::Game;

fn main() {
    let (g2u_tx, g2u_rx) = mpsc::channel::<helper::G2UMessage>();
    let (u2g_tx, u2g_rx) = mpsc::channel::<helper::U2GMessage>();

    std::thread::spawn(move || {
        let mut window = ui::Ui::new(UiSettings {
            title: String::from(settings::WINDOW_TITLE),
            width: settings::WINDOW_WIDTH,
            height: settings::WINDOW_HEIGHT,
        });

        window.run(&u2g_tx, &g2u_rx);
    });

    let mut game = Game::new();

    game.init(&g2u_tx, &u2g_rx);
}
