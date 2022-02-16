use crate::settings;
use crate::physics;
use crate::rectangle::{Rectangle, Size, RectangleSize};
use physics::{Position, ChangingFactor};
use crate::game::Game;
use crate::asteroid::Asteroid;

// Missile Types
pub mod missiles;

pub struct MissileData {
    pub initial_velocity: ChangingFactor,
    pub direction: ChangingFactor,
    pub acceleration: ChangingFactor,
    pub delay: u128,
}

pub type CollisionHandler = fn (&mut Game, &mut Missile, &mut Asteroid); 

#[derive(Clone, Copy)]
pub enum MissileType {
    Normal,
    Bomb,
}

#[derive(Clone, Copy)]
pub struct Missile {
    pub position: Position,
    pub velocity: ChangingFactor,
    pub direction: ChangingFactor,
    pub acceleration: ChangingFactor,
    pub active: bool,
    pub missile_type: MissileType,
    pub collision_handler: CollisionHandler,
}

impl Missile {
    pub fn update_missiles_position(missiles: &mut Vec<Missile>) {
        let mut updated_missiles = vec![];
        
        for missile in missiles.iter_mut() {
            if !missile.active {
                updated_missiles.push(*missile);    
                continue;
            }
            let corners = Rectangle {
                position: Position {
                    x: missile.position.x,
                    y: missile.position.y,
                },
                size: Size::Rectangle(RectangleSize {
                    width: settings::MISSILE_WIDTH,
                    height: settings::MISSILE_HEIGHT,
                }),
            }.get_corners();

            if corners.top_left.x > settings::WINDOW_WIDTH as i32
            || corners.top_left.x < 0
            || corners.top_left.y > settings::WINDOW_HEIGHT as i32
            || corners.top_left.y < 0{
                continue;
            }

            
            missile.position = physics::next_position(
                Rectangle {
                    position: missile.position,
                    size: Size::Rectangle(RectangleSize {
                        width: settings::MISSILE_WIDTH,
                        height: settings::MISSILE_HEIGHT,
                    }),
                },
                missile.velocity,
                vec![
                    physics::Force {
                        direction: missile.direction,
                        acceleration: missile.acceleration,
                    }
                ]
            );

            updated_missiles.push(*missile);
        }

        *missiles = updated_missiles;
    }

    pub fn get_types_data(missile_type: MissileType) -> MissileData {
        match missile_type {
            MissileType::Normal => missiles::Normal::get_missile_data(),
            MissileType::Bomb => missiles::Bomb::get_missile_data(),
        }
    }

    pub fn get_types_handler(missile_type: MissileType) -> CollisionHandler {
        match missile_type {
            MissileType::Normal => missiles::Normal::collision_handler,
            MissileType::Bomb => missiles::Bomb::collision_handler,
        }
    }
}