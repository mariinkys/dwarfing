use macroquad::{math::Rect, texture::Texture2D};

use crate::{score::Score, shape::Shape};

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
    Gold {
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
    pub fn new(shape: Shape, block_type: BlockType) -> Self {
        Self { shape, block_type }
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
            }
            | BlockType::Gold {
                hp,
                base_hp: _,
                texture: _,
            } => *hp <= 0,
        }
    }

    pub fn subtract_block_hp(&mut self, hp_to_subtract: i32) -> bool {
        match &mut self.block_type {
            BlockType::Dirt { hp, .. }
            | BlockType::Rock { hp, .. }
            | BlockType::Gold { hp, .. } => {
                // TODO: Depending on the player upgrades we should subtract more or less hp?
                *hp -= hp_to_subtract;
                if *hp <= 0 {
                    *hp = 0;
                    return true;
                }
                false
            }
        }
    }

    // Select the correct texture based on the block hp relative to the base hp
    pub fn texture_selector(&self) -> Rect {
        match &self.block_type {
            BlockType::Dirt { hp, base_hp, .. }
            | BlockType::Rock { hp, base_hp, .. }
            | BlockType::Gold { hp, base_hp, .. } => {
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

    pub fn update_score(&self, score: &mut Score) {
        score.blocks_destroyed += 1;
        match self.block_type {
            BlockType::Dirt {
                base_hp: _,
                hp: _,
                texture: _,
            } => {
                score.current_score += 1;
                score.gold += 1;
            }
            BlockType::Rock {
                base_hp: _,
                hp: _,
                texture: _,
            } => {
                score.current_score += 3;
                score.gold += 3;
            }
            BlockType::Gold {
                base_hp: _,
                hp: _,
                texture: _,
            } => {
                score.current_score += 10;
                score.gold += 10;
            }
        }
    }
}
