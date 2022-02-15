#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}

pub fn next_position(
    rectangle_position: Position,
    direction: Direction,
    acceleration: Direction,
) -> Position {
    let next_pos = Position {
        x: (rectangle_position.x as f32 + direction.x * acceleration.x) as i32,
        y: (rectangle_position.y as f32 + direction.y * acceleration.y) as i32,
    };

    return next_pos;
}