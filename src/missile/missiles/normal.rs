use crate::physics::{Direction};

use crate::time;
use crate::missile::MissileData;

pub struct Normal {}

impl Normal {
    pub fn get_missile_data() -> MissileData {
        return MissileData {
            direction: Direction {
                x: 1.0,
                y: 0.0,
            },
            acceleration: Direction {
                x: 10_f32,
                y: 0_f32,
            },
            delay: time::to_nano(1000 / 15),
        }
    }
}