use macroquad::texture::{build_textures_atlas, load_texture, FilterMode, Texture2D};

#[derive(Clone, Debug)]
pub struct Resources {
    pub player_texture: Texture2D,
    pub dirt_block_texture: Texture2D,
}

impl Resources {
    pub async fn new() -> Self {
        let player_texture: Texture2D = load_texture("sprites/player.png")
            .await
            .expect("Can't load player texture");
        player_texture.set_filter(FilterMode::Nearest);

        let dirt_block_texture: Texture2D = load_texture("blocks/dirt.png")
            .await
            .expect("Can't load dirt block texture");
        dirt_block_texture.set_filter(FilterMode::Nearest);

        // This will ensure that all calls to draw_texture() and draw_texture_ex() will use the texture from the atlas instead of each separate texture,
        // which is much more efficient. All textures need to be loaded before this function is called.
        build_textures_atlas();

        Self {
            player_texture,
            dirt_block_texture,
        }
    }
}
