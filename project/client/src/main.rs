mod app;
mod button;
mod grid;
mod menu;
mod network;
mod notification;

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

    match App::new() {
        Ok(mut app) => {
            app.menu.load_textures().await;

            loop {
                clear_background(Color::from_hex(0x3B4953));
                app.check_resize();
                if let Err(e) = app.update_state().await {
                    eprintln!("Error [{}]", e);
                }
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
        Err(e) => loop {
            clear_background(Color::from_hex(0x3B4953));
            draw_text(
                (get_fps().to_string() + " fps").as_str(),
                16.0,
                32.0,
                32.0,
                WHITE,
            );
            draw_text(
                format!("Could not connect to server ({})", e).as_str(),
                16.0,
                screen_height() * 0.5,
                24.0,
                WHITE,
            );
            next_frame().await
        },
    }
}
