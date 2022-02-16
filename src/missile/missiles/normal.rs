use crate::physics::{ChangingFactor};

use crate::time;
use crate::missile::{MissileData, Missile};
use crate::game::Game;
use crate::asteroid::Asteroid;

pub struct Normal {}

impl Normal {
    pub fn get_missile_data() -> MissileData {
        return MissileData {
            initial_velocity: ChangingFactor {
                x: 10.0,
                y: 0.0,
            },
            direction: ChangingFactor {
                x: 1.0,
                y: 0.0,
            },
            acceleration: ChangingFactor {
                x: 0.0,
                y: 0.0,
            },
            delay: time::to_nano(1000 / 15),
        }
    }

    pub fn collision_handler(_game: &mut Game, missile: &mut Missile, asteroid: &mut Asteroid) {
        (*asteroid).size -= 1;
        (*missile).active = false;
    }
}