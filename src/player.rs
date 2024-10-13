use crate::shape::Shape;

const MOVEMENT_SPEED: f32 = 1.0;

pub struct Player {
    pub shape: Shape,
    pub speed: f32,
    pub offset_y: f32, // Add this field to track vertical offset, to fix gravity jankiness
}

impl Player {
    pub fn new(shape: Shape) -> Self {
        let speed = MOVEMENT_SPEED;
        Self {
            shape,
            speed,
            offset_y: 0.0,
        }
    }
}
