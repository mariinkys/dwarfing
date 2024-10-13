use macroquad::prelude::*;

mod block;
mod dwarfing;
mod player;
mod resources;
mod shape;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Dwarfing"),
        fullscreen: true,
        window_resizable: false,
        window_width: 1024,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let mut game = dwarfing::Dwarfing::init().await;

    loop {
        clear_background(LIGHTGRAY);

        game.update();
        game.draw();

        next_frame().await;
    }
}
