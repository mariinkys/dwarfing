use crate::shape::Shape;

pub enum BlockType {
    Dirt {
        hp: i32,
        // TODO: Maybe the texture should be set here?
        //texture: Texture2D
    },
    Rock {
        hp: i32,
    },
}

pub struct Block {
    pub shape: Shape,
    pub block_type: BlockType,
    // TODO: The block texture should be part of the block state
    //pub texture: Texture2D,
}

impl Block {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            block_type: BlockType::Dirt { hp: 50 },
            //texture,
        }
    }

    pub fn is_destroyed(&self) -> bool {
        match &self.block_type {
            BlockType::Dirt { hp } | BlockType::Rock { hp } => *hp <= 0,
        }
    }
}
