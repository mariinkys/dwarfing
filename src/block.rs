use macroquad::{math::Rect, texture::Texture2D};

use crate::shape::Shape;

pub enum BlockType {
    Dirt {
        base_hp: i32,
        hp: i32,
        texture: Texture2D,
    },
    Rock {
        base_hp: i32,
        hp: i32,
        texture: Texture2D,
    },
}

pub struct Block {
    pub shape: Shape,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(shape: Shape, texture: Texture2D) -> Self {
        Self {
            shape,
            block_type: BlockType::Dirt {
                base_hp: 50,
                hp: 50,
                texture,
            },
            //texture,
        }
    }

    pub fn is_destroyed(&self) -> bool {
        match &self.block_type {
            BlockType::Dirt {
                hp,
                base_hp: _,
                texture: _,
            }
            | BlockType::Rock {
                hp,
                base_hp: _,
                texture: _,
            } => *hp <= 0,
        }
    }

    pub fn subtract_block_hp(&mut self) {
        match &mut self.block_type {
            BlockType::Dirt { hp, .. } | BlockType::Rock { hp, .. } => {
                // TODO: Depending on the player upgrades we should subtract more or less hp?
                *hp -= 10;
                if *hp < 0 {
                    *hp = 0;
                }
            }
        }
    }

    // Select the correct texture based on the block hp relative to the base hp
    pub fn texture_selector(&self) -> Rect {
        match &self.block_type {
            BlockType::Dirt { hp, base_hp, .. } | BlockType::Rock { hp, base_hp, .. } => {
                let hp_percentage = (*hp as f32 / *base_hp as f32) * 100.0;
                let column = if hp_percentage >= 75.0 {
                    1
                } else if hp_percentage >= 50.0 {
                    2
                } else if hp_percentage >= 25.0 {
                    3
                } else {
                    4
                };

                // Example code
                //Rect {
                //    x: 32.0, // Column 1, of a 32x32
                //    //x = 128.0, // Column 4, of a 32x32
                //    y: 0.0,  // Row 0
                //    w: 32.0, // Width of the frame
                //    h: 32.0, // Height of the frame
                //}

                Rect {
                    x: (column - 1) as f32 * 32.0, // Column selection
                    y: 0.0,                        // Row 0
                    w: 32.0,                       // Width of the frame
                    h: 32.0,                       // Height of the frame
                }
            }
        }
    }
}
