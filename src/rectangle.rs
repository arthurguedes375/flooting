use crate::helper::Position;

pub struct RectangleCornersPositions {
    pub top_left: Position,
    pub top_right: Position,
    pub bottom_left: Position,
    pub bottom_right: Position,
}

#[derive(Clone)]
pub struct RectangleSize {
    pub height: u32,
    pub width: u32,
}

#[derive(Clone)]
pub enum Size {
    Square(u32),
    Rectangle(RectangleSize),
}

#[derive(Clone)]
pub struct Rectangle {
    pub position: Position,
    pub size: Size,
}

impl Rectangle {

    pub fn to_rectangle_size(size: Size) -> RectangleSize {
        let final_size: RectangleSize;
        match size {
            Size::Square(square_size) => {
                final_size = RectangleSize {
                    width: square_size,
                    height: square_size,
                }
            }
            Size::Rectangle(rectangle_size) => {
                final_size = rectangle_size;
            }
        }

        return final_size;
    }

    pub fn get_corners(&self) -> RectangleCornersPositions {
        let size = Rectangle::to_rectangle_size(self.size.clone());
        let corners = RectangleCornersPositions {
            top_left: Position {
                x: self.position.x - size.width as i32 / 2,
                y: self.position.y - size.height as i32 / 2,
            },
            top_right: Position {
                x: self.position.x + size.width as i32 / 2,
                y: self.position.y - size.height as i32 / 2,
            },
            bottom_left: Position {
                x: self.position.x - size.width as i32 / 2,
                y: self.position.y + size.height as i32 / 2,
            },
            bottom_right: Position {
                x: self.position.x + size.width as i32 / 2,
                y: self.position.y + size.height as i32 / 2,
            },
        };
        
        return corners;
    }

    pub fn contains_position(&self, position: Position) -> bool {
        let corners = self.get_corners();

        if position.x > corners.top_left.x
        && (position.x as i64) < corners.top_right.x as i64
        && position.y > corners.top_left.y
        && (position.y as i64) < corners.bottom_left.y as i64 {
            return true;
        }
        return false;
    }

    pub fn over(&self, outside_rectangle: Rectangle) -> bool {
        let corners = self.get_corners();

        if outside_rectangle.contains_position(corners.top_left)
        || outside_rectangle.contains_position(corners.top_right)
        || outside_rectangle.contains_position(corners.bottom_left)
        || outside_rectangle.contains_position(corners.bottom_right) {
            return true;
        }

        return false;
    }

}