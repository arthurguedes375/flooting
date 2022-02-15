use crate::physics::ChangingFactor;

#[derive(Clone, Copy, Debug)]
pub struct Force {
    pub direction: ChangingFactor,
    pub acceleration: ChangingFactor,
}

impl Force {
    pub fn add_force(&mut self, force: Force) {
        self.direction = ChangingFactor {
            x: self.direction.x + force.direction.x,
            y: self.direction.y + force.direction.y,
        };
        self.acceleration = ChangingFactor {
            x: self.acceleration.x + force.acceleration.x,
            y: self.acceleration.y + force.acceleration.y,
        }
    }
}