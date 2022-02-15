pub mod force;
pub use force::Force;

use crate::rectangle::Rectangle;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct ChangingFactor {
    pub x: f32,
    pub y: f32,
}

pub fn next_position(
    rectangle: Rectangle,
    current_velocity: ChangingFactor,
    forces: Vec<Force>,
) -> Position {
    let mut resultant_force = Force {
        direction: ChangingFactor {
            x: 0.0,
            y: 0.0,
        },
        acceleration: ChangingFactor {
            x: 0.0,
            y: 0.0,
        }
    };

    for &force in forces.iter() {
        resultant_force.add_force(force);
    }

    let rectangle_size = Rectangle::to_rectangle_size(rectangle.size);

    let next_pos = Position {
        x: (
            rectangle.position.x as f32
            + current_velocity.x * resultant_force.direction.x
            + (resultant_force.direction.x * resultant_force.acceleration.x) / rectangle_size.width as f32
        ) as i32,
        y: (
            rectangle.position.y as f32
            + current_velocity.y * resultant_force.direction.y
            + (resultant_force.direction.y * resultant_force.acceleration.y) / rectangle_size.height as f32
        ) as i32,
    };

    return next_pos;
}