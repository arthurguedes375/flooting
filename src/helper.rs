use sdl2::event::Event;

use crate::physics::Position;
use crate::game::Game;

pub enum G2UMessage {
    StateUpdate(Game)
}

pub enum U2GMessage {
    MouseMotion(Position),
    Event(Event),
    Close,
}