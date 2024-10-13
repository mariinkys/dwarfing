use crate::shape::Shape;

pub struct Block {
    pub shape: Shape,
    pub destroyed: bool,
}

impl Block {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            destroyed: false,
        }
    }
}
