mod app;
mod grid;
mod menu;
mod network;

use crate::app::App;
use macroquad::prelude::*;

fn get_conf() -> Conf {
    Conf {
        window_width: 1048,
        window_height: 840,
        window_title: "Trap the Mouse".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(get_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    let mut app = App::new();

    
    loop {
        clear_background(Color::from_hex(0x3B4953));
        app.check_resize();
        app.handle_input();

        app.render();

        draw_text(
            (get_fps().to_string() + " fps").as_str(),
            16.0,
            32.0,
            32.0,
            WHITE,
        );
        next_frame().await
    }
}
