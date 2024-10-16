use ::rand::Rng;
use macroquad::{audio::stop_sound, prelude::*};

use crate::{
    block::{Block, BlockType},
    player::Player,
    resources::Resources,
    score::Score,
    shape::Shape,
};

const GRAVITY: f32 = 500.0;
const BLOCK_SIZE: f32 = 32.0;

#[derive(Debug, PartialEq)]
enum DebugMode {
    Enabled,
    Disabled,
}

#[derive(Debug, PartialEq)]
enum PlayingThemeAudio {
    OpeningTheme,
    GameTheme,
}

struct Params {
    needed_x: i32,
    block_area_top: f32,
    last_row_y: f32,
}

pub struct Dwarfing {
    debug_mode: DebugMode,
    score: Score,
    playing_theme: PlayingThemeAudio,
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

        let score = Score::init();

        let player = Player::new(player_shape, resources.player_texture.clone());
        let blocks = Vec::new();

        let needed_x = (screen_width() / BLOCK_SIZE).ceil() as i32;
        let block_area_top = screen_height() / 2.0;
        let last_row_y = block_area_top;

        let playing_theme = PlayingThemeAudio::OpeningTheme;

        Self {
            debug_mode: DebugMode::Disabled,
            score,
            playing_theme,
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
        self.init_music();
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

        set_default_camera();
        if self.debug_mode == DebugMode::Enabled {
            Self::draw_debug_info(&self.player, &self.blocks);
        }
        self.draw_ui();
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
            Self::spawn_row_of_blocks(
                &mut self.blocks,
                self.params.needed_x,
                new_row_y,
                &self.resources,
            );
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
        if is_mouse_button_down(MouseButton::Left) {
            self.player.sprite.set_animation(1);
            if is_mouse_button_pressed(MouseButton::Left) {
                Self::destroy_touching_blocks(&mut self.blocks, &self.player, &mut self.score);
            }
        } else {
            self.player.sprite.set_animation(0);
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

                        let texture = match &block.block_type {
                            crate::block::BlockType::Dirt {
                                base_hp: _,
                                hp: _,
                                texture,
                            } => texture,
                            crate::block::BlockType::Rock {
                                base_hp: _,
                                hp: _,
                                texture,
                            } => texture,
                            crate::block::BlockType::Gold {
                                base_hp: _,
                                hp: _,
                                texture,
                            } => texture,
                        };

                        draw_texture_ex(
                            texture,
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

    fn draw_ui(&self) {
        let score_text = format!("Score = {}", &self.score.current_score);
        draw_text(
            score_text.as_str(),
            screen_width() - 10.0 - measure_text(score_text.as_str(), None, 20, 1.0).width,
            20.0,
            20.0,
            BLACK,
        );

        let destroyed_blocks_text = format!("Destroyed Blocks = {}", &self.score.blocks_destroyed);
        draw_text(
            destroyed_blocks_text.as_str(),
            screen_width()
                - 10.0
                - measure_text(destroyed_blocks_text.as_str(), None, 20, 1.0).width,
            45.0,
            20.0,
            BLACK,
        );
    }

    //
    // HELPERS
    //

    fn spawn_row_of_blocks(blocks: &mut Vec<Block>, needed_x: i32, y: f32, resources: &Resources) {
        for x in 0..needed_x {
            let shape = Shape {
                x: x as f32 * BLOCK_SIZE,
                y,
                size: Vec2::splat(BLOCK_SIZE),
                color: RED,
            };

            // TODO: Move this to a helper function
            // Select a Random Block
            let dynamic_rock_cap = (0.3 + (0.001 * (y - 1000.0).max(0.0))).min(0.8); // Cap increases after y = 1000, maxing at 80%
            let rock_probability = (0.01 * y.ln()).min(dynamic_rock_cap); // Rock probability increases with depth

            let dynamic_gold_cap = (0.02 + (0.001 * (y - 2000.0).max(0.0))).min(0.2); // Cap increases slowly, maxing at 20%
            let gold_probability = if y > 2000.0 {
                (0.002 * (y - 2000.0).ln()).min(dynamic_gold_cap) // Gold probability increases after y = 2000
            } else {
                0.0 // No gold above y = 2000
            };

            // TODO: This should be initialaized with the game and kept in the game state?
            let mut rng = ::rand::thread_rng();
            let rng_num = rng.gen::<f32>();

            let block_type = if rng_num < gold_probability {
                BlockType::Gold {
                    base_hp: 100,
                    hp: 100,
                    texture: resources.gold_block_texture.clone(), // TODO: Can I avoid cloning the textures?
                }
            } else if rng_num < (rock_probability + gold_probability) {
                BlockType::Rock {
                    base_hp: 70,
                    hp: 70,
                    texture: resources.rock_block_texture.clone(),
                }
            } else {
                BlockType::Dirt {
                    base_hp: 50,
                    hp: 50,
                    texture: resources.dirt_block_texture.clone(),
                }
            };

            blocks.push(Block::new(shape, block_type));
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

    fn destroy_touching_blocks(blocks: &mut [Block], player: &Player, score: &mut Score) {
        for block in blocks.iter_mut() {
            if !block.is_destroyed() && Self::check_collision(&player.shape, &block.shape) {
                let block_destroyed = block.subtract_block_hp();
                if block_destroyed {
                    block.update_score(score);
                }
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

    fn init_music(&mut self) {
        if self.playing_theme == PlayingThemeAudio::OpeningTheme {
            self.playing_theme = PlayingThemeAudio::GameTheme;
            stop_sound(&self.resources.opening_theme);
            macroquad::audio::play_sound(
                &self.resources.game_theme,
                macroquad::audio::PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
        }
    }
}
