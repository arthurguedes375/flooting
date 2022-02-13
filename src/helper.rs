use sdl2::event::Event;

use crate::game::Game;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub enum G2UMessage {
    StateUpdate(Game)
}

pub enum U2GMessage {
    MouseMotion(Position),
    Event(Event),
    Close,
}