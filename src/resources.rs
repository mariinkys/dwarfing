use macroquad::{
    color::WHITE,
    math::RectOffset,
    texture::{build_textures_atlas, load_image, load_texture, FilterMode, Image, Texture2D},
    ui::{root_ui, Skin},
};

#[derive(Clone, Debug)]
pub struct Resources {
    // Textures
    pub player_texture: Texture2D,
    pub dirt_block_texture: Texture2D,
    pub rock_block_texture: Texture2D,
    pub gold_block_texture: Texture2D,

    // Images
    pub menu_background: Image,
    pub button_background: Image,
    pub button_clicked_background: Image,
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

        let rock_block_texture: Texture2D = load_texture("blocks/stone.png")
            .await
            .expect("Can't load rock block texture");
        dirt_block_texture.set_filter(FilterMode::Nearest);

        let gold_block_texture: Texture2D = load_texture("blocks/gold.png")
            .await
            .expect("Can't load gold block texture");
        dirt_block_texture.set_filter(FilterMode::Nearest);

        // This will ensure that all calls to draw_texture() and draw_texture_ex() will use the texture from the atlas instead of each separate texture,
        // which is much more efficient. All textures need to be loaded before this function is called.
        build_textures_atlas();

        let menu_background = load_image("resources/main_menu_bg.png").await.unwrap();
        let button_background = load_image("resources/button_background.png").await.unwrap();
        let button_clicked_background = load_image("resources/button_clicked_background.png")
            .await
            .unwrap();

        Self {
            player_texture,
            dirt_block_texture,
            rock_block_texture,
            gold_block_texture,
            menu_background,
            button_background,
            button_clicked_background,
        }
    }

    pub fn build_ui(self) {
        let window_style = root_ui()
            .style_builder()
            .background(self.menu_background)
            .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
            .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(self.button_background)
            .background_clicked(self.button_clicked_background)
            .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
            //.font(&font)
            //.unwrap()
            .text_color(WHITE)
            .font_size(64)
            .build();

        let label_style = root_ui()
            .style_builder()
            //.font(&font)
            //.unwrap()
            .text_color(WHITE)
            .font_size(28)
            .build();

        let ui_skin = Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&ui_skin);
    }
}
