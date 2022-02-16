use crate::physics::{ChangingFactor};
use crate::missile::{MissileData, Missile};
use crate::game::Game;
use crate::time;
use crate::asteroid::Asteroid;
use crate::missile;
use missile::MissileType;
use missile::missiles::Normal;



pub struct Bomb {}

impl Bomb {
    pub fn get_missile_data() -> MissileData {
        return MissileData {
            initial_velocity: ChangingFactor {
                x: 3.0,
                y: 0.0,
            },
            direction: ChangingFactor {
                x: 1.0,
                y: 0.0,
            },
            acceleration: ChangingFactor {
                x: 20.0,
                y: 0.0,
            },
            delay: time::to_nano(1000 / 10),
        }
    }

    pub fn collision_handler(game: &mut Game, missile: &mut Missile, asteroid: &mut Asteroid) {
        (*missile).active = false;
        (*asteroid).size -= 1;

        let lookup_directions_table = [
            (-1.0, -1.0),
            (0.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ];

        for missile_i in 0..6 {
            let next_missile = Missile {
                active: true,
                velocity: ChangingFactor {
                    x: Normal::get_missile_data().initial_velocity.x / 2.0,
                    y: Normal::get_missile_data().initial_velocity.x / 2.0,
                },
                acceleration: ChangingFactor {
                    x: 0.0,
                    y: 0.0,
                },
                direction: ChangingFactor {
                    x: lookup_directions_table[missile_i].0,
                    y: lookup_directions_table[missile_i].1,
                },
                collision_handler: Normal::collision_handler,
                missile_type: MissileType::Normal,
                position: missile.position,
            };
            
            if missile_i == 0 {
                *missile = next_missile;
            } else {
                game.missiles.push(next_missile);
            }
        }
    }
}