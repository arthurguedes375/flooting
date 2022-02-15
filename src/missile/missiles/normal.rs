use crate::physics::{ChangingFactor};

use crate::time;
use crate::missile::MissileData;

pub struct Normal {}

impl Normal {
    pub fn get_missile_data() -> MissileData {
        return MissileData {
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
}