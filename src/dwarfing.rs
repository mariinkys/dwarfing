use macroquad::prelude::*;

use crate::{block::Block, player::Player, resources::Resources, shape::Shape};

const GRAVITY: f32 = 500.0;
const BLOCK_SIZE: f32 = 32.0;

#[derive(Debug, PartialEq)]
enum DebugMode {
    Enabled,
    Disabled,
}

struct Params {
    needed_x: i32,
    block_area_top: f32,
    last_row_y: f32,
}

pub struct Dwarfing {
    debug_mode: DebugMode,
    resources: Resources,
    player: Player,
    blocks: Vec<Block>,
    params: Params,
}

impl Dwarfing {
    pub fn init(resources: Resources) -> Self {
        let player_shape = Shape {
            x: screen_width() / 2.0,
            y: 0.0,
            size: Vec2::splat(32.0),
            color: BLUE,
        };

        let player = Player::new(player_shape, resources.player_texture.clone());
        let blocks = Vec::new();

        let needed_x = (screen_width() / BLOCK_SIZE).ceil() as i32;
        let block_area_top = screen_height() / 2.0;
        let last_row_y = block_area_top;

        Self {
            debug_mode: DebugMode::Disabled,
            resources,
            player,
            blocks,
            params: Params {
                needed_x,
                block_area_top,
                last_row_y,
            },
        }
    }

    pub fn update(&mut self) {
        self.apply_gravity();
        self.update_player_position();
        self.update_blocks();
        self.remove_off_screen_blocks();
        self.player_collision();
        self.handle_input();
        self.handle_camera();
    }

    pub fn draw(&self) {
        self.draw_blocks();
        self.draw_player();

        if self.debug_mode == DebugMode::Enabled {
            set_default_camera();
            Self::draw_debug_info(&self.player, &self.blocks);
        }
    }

    //
    // UPDATE FUNCTIONS
    //

    fn apply_gravity(&mut self) {
        let delta_time = get_frame_time();
        self.player.speed += GRAVITY * delta_time;
        self.player.offset_y -= self.player.speed * delta_time;
    }

    fn update_player_position(&mut self) {
        // The player should always be at the center, the drawing function draws from the top-left corner
        self.player.shape.x = screen_width() / 2.0 - self.player.shape.size.x / 2.0;
        //player.shape.y = block_area_top - player.shape.size.y; // The player should be above the blocks sowe subtract the player size.
        self.player.shape.y =
            self.params.block_area_top - self.player.shape.size.y - self.player.offset_y;
        // Take into account the gravity offset
    }

    fn update_blocks(&mut self) {
        // Generate new blocks if needed
        if self.player.shape.y + screen_height() > self.params.last_row_y {
            let new_row_y = self.params.last_row_y + BLOCK_SIZE;
            Self::spawn_row_of_blocks(&mut self.blocks, self.params.needed_x, new_row_y);
            self.params.last_row_y = new_row_y;
        }
    }

    fn remove_off_screen_blocks(&mut self) {
        self.blocks
            .retain(|block| block.shape.y > self.player.shape.y - screen_height());
    }

    fn player_collision(&mut self) {
        // Collision detection and resolution
        for block in &self.blocks {
            if !block.is_destroyed() && Self::check_collision(&self.player.shape, &block.shape) {
                Self::resolve_collision(&mut self.player, &block.shape, self.params.block_area_top);
            }
        }
    }

    fn handle_input(&mut self) {
        self.player.sprite.set_animation(0);
        if is_mouse_button_pressed(MouseButton::Left) {
            self.player.sprite.set_animation(1);
            Self::destroy_touching_blocks(&mut self.blocks, &self.player);
        }
        self.player.sprite.update();
    }

    fn handle_camera(&self) {
        let mut camera = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: screen_width(),
            h: screen_height(),
        });

        // Fix the Y axis flipping and re-center the camera
        camera.zoom = vec2(1. / screen_width() * 2., 1. / screen_height() * 2.);

        // Manually center the camera on the middle of the screen
        camera.target = vec2(screen_width() / 2.0, self.player.shape.y);

        set_camera(&camera);
    }

    //
    // DRAW FUNCTIONS
    //

    fn draw_blocks(&self) {
        for block in &self.blocks {
            if !block.is_destroyed() {
                match self.debug_mode {
                    DebugMode::Enabled => {
                        draw_rectangle_lines(
                            block.shape.x,
                            block.shape.y,
                            block.shape.size.x,
                            block.shape.size.y,
                            1.0,
                            block.shape.color,
                        );
                    }
                    DebugMode::Disabled => {
                        //draw_rectangle(
                        //    block.shape.x,
                        //    block.shape.y,
                        //    block.shape.size.x,
                        //    block.shape.size.y,
                        //    block.shape.color,
                        //);

                        // TODO: Maybe the block texture should be part of the block state
                        draw_texture_ex(
                            &self.resources.dirt_block_texture,
                            block.shape.x,
                            block.shape.y,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2 {
                                    x: block.shape.size.x,
                                    y: block.shape.size.y,
                                }),
                                source: Some(block.texture_selector()),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    }

    fn draw_player(&self) {
        match self.debug_mode {
            DebugMode::Enabled => {
                draw_rectangle(
                    self.player.shape.x,
                    self.player.shape.y,
                    self.player.shape.size.x,
                    self.player.shape.size.y,
                    self.player.shape.color,
                );
            }
            DebugMode::Disabled => {
                let frame = self.player.sprite.frame();

                draw_texture_ex(
                    &self.player.texture,
                    self.player.shape.x,
                    self.player.shape.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2 {
                            x: self.player.shape.size.x,
                            y: self.player.shape.size.y,
                        }),
                        source: Some(frame.source_rect),
                        ..Default::default()
                    },
                );
            }
        }
    }

    //
    // HELPERS
    //

    fn spawn_row_of_blocks(blocks: &mut Vec<Block>, needed_x: i32, y: f32) {
        for x in 0..needed_x {
            let shape = Shape {
                x: x as f32 * BLOCK_SIZE,
                y,
                size: Vec2::splat(BLOCK_SIZE),
                color: RED,
            };
            blocks.push(Block::new(shape));
        }
    }

    fn check_collision(a: &Shape, b: &Shape) -> bool {
        a.x < b.x + b.size.x && a.x + a.size.x > b.x && a.y < b.y + b.size.y && a.y + a.size.y > b.y
    }

    fn resolve_collision(player: &mut Player, block: &Shape, block_area_top: f32) {
        let player_bottom = player.shape.y + player.shape.size.y;
        let block_top = block.y;

        if player_bottom > block_top {
            player.offset_y = block_area_top - block_top;
            player.speed = 0.0;
        }
    }

    fn destroy_touching_blocks(blocks: &mut [Block], player: &Player) {
        for block in blocks.iter_mut() {
            if !block.is_destroyed() && Self::check_collision(&player.shape, &block.shape) {
                block.subtract_block_hp();
                return; // Added a return so you only break one block per click
            }
        }
    }

    fn draw_debug_info(player: &Player, blocks: &[Block]) {
        let player_text = format!(
            "Player Position = x:{:.2} y:{:.2}",
            player.shape.x, player.shape.y
        );
        draw_text(player_text.as_str(), 10.0, 20.0, 20.0, BLACK);

        let destroyed_blocks = blocks.iter().filter(|block| block.is_destroyed()).count();
        let total_blocks = blocks.len();
        let block_text = format!(
            "Number of blocks = Alive:{} Destroyed:{} TOTAL:{}",
            total_blocks - destroyed_blocks,
            destroyed_blocks,
            total_blocks
        );
        draw_text(block_text.as_str(), 10.0, 45.0, 20.0, BLACK);
    }
}
