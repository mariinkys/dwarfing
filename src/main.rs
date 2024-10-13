use macroquad::prelude::*;

mod block;
mod dwarfing;
mod player;
mod shape;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("MacroQUADNTS"),
        fullscreen: true,
        window_resizable: false,
        window_width: 1024,
        window_height: 800,
        //icon: Some(icon::set()),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = dwarfing::Dwarfing::init();

    loop {
        clear_background(LIGHTGRAY);

        game.update();
        game.draw();

        next_frame().await;
    }
}
