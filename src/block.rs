use crate::shape::Shape;

pub struct Block {
    pub shape: Shape,
    pub destroyed: bool,
    // TODO: The block texture should be part of the block state
    //pub texture: Texture2D,
}

impl Block {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            destroyed: false,
            //texture,
        }
    }
}
