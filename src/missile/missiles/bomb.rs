use crate::physics::{ChangingFactor};

use crate::missile::MissileData;

use crate::time;

pub struct Bomb {}

impl Bomb {
    pub fn get_missile_data() -> MissileData {
        return MissileData {
            direction: ChangingFactor {
                x: 0_f32,
                y: 0_f32,
            },
            acceleration: ChangingFactor {
                x: 0_f32,
                y: 0_f32,
            },
            delay: time::to_nano(1000 / 3),
        }
    }
}