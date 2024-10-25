use macroquad::{
    prelude::animation::{AnimatedSprite, Animation},
    texture::Texture2D,
};

use crate::shape::Shape;

const MOVEMENT_SPEED: f32 = 1.0;

#[derive(PartialEq)]
pub enum Pickaxe {
    Normal,
    Iron,
    Gold,
}

pub struct Player {
    pub shape: Shape,
    pub speed: f32,
    pub offset_y: f32, // Add this field to track vertical offset, to fix gravity jankiness
    pub current_pickaxe: Pickaxe,

    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
}

impl Player {
    pub fn new(shape: Shape, texture: Texture2D) -> Self {
        let speed = MOVEMENT_SPEED;

        let sprite = AnimatedSprite::new(
            32,
            32,
            &[
                Animation {
                    name: String::from("idle"),
                    row: 0,
                    frames: 2,
                    fps: 5,
                },
                Animation {
                    name: String::from("work"),
                    row: 1,
                    frames: 3,
                    fps: 5,
                },
            ],
            true,
        );

        Self {
            shape,
            speed,
            offset_y: 0.0,
            sprite,
            texture,
            current_pickaxe: Pickaxe::Normal,
        }
    }

    pub fn swap_texture(&mut self, texture: Texture2D) {
        self.texture = texture;
    }
}
