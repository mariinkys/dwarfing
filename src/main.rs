use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};
use resources::Resources;

mod block;
mod dwarfing;
mod player;
mod resources;
mod shape;

const WINDOW_SIZE_X: i32 = 1056;
const WINDOW_SIZE_Y: i32 = 800;

enum GameState {
    Menu,
    Playing,
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Dwarfing"),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW_SIZE_X,
        window_height: WINDOW_SIZE_Y,
        ..Default::default()
    }
}

fn menu_ui() -> GameState {
    let mut state = GameState::Menu;

    let window_width = screen_width();
    let window_height = screen_height();

    // Define UI element sizes
    let button_width = 200.0;
    let button_height = 50.0;
    let label_height = 40.0;
    let vertical_spacing = 50.0;

    // Calculate total height of all elements
    let total_height = label_height + (button_height * 2.0) + (vertical_spacing * 2.0);

    // Calculate starting Y position to center everything vertically
    let start_y = (window_height - total_height) / 2.0;

    root_ui().window(
        hash!(),
        vec2(0.0, 0.0),
        vec2(window_width, window_height),
        |ui| {
            let play_button_x = (window_width - button_width) / 2.0;
            let play_button_y = start_y + label_height + vertical_spacing;
            if ui.button(vec2(play_button_x, play_button_y), "Play") {
                state = GameState::Playing;
            }

            let quit_button_x = (window_width - button_width) / 2.0;
            let quit_button_y = play_button_y + button_height + vertical_spacing;
            if ui.button(vec2(quit_button_x, quit_button_y), "Quit") {
                std::process::exit(0);
            }
        },
    );
    state
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let resources = Resources::new().await;
    resources.clone().build_ui(); // TODO: Can I avoid cloning here?

    let mut game_state = GameState::Menu;
    let mut game = dwarfing::Dwarfing::init(resources);

    loop {
        clear_background(LIGHTGRAY);

        match game_state {
            GameState::Menu => {
                game_state = menu_ui();
            }
            GameState::Playing => {
                game.update();
                game.draw();
            }
        }

        next_frame().await;
    }
}
