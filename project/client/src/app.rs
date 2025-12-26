use std::{thread, time};

use crate::{
    grid::Grid,
    menu::Menu,
    network::{Network, Update},
};

use macroquad::{
    color::Color,
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Vec2,
    miniquad::window::screen_size,
    text::draw_text,
    window::{clear_background, next_frame, screen_height, screen_width},
};

const MIN_WIDTH: f32 = 64.0;
const MIN_HEIGHT: f32 = 64.0;

#[derive(thiserror::Error, Debug)]
pub enum ClientErr {
    #[error("Invalid username!")]
    InvalidUsername,

    #[error("Join failed!")]
    JoinFail,

    #[error("Io error: {0}")]
    IO(#[from] std::io::Error),
}

pub struct App {
    pub menu: Menu,
    grid: Option<Grid>,
    network: Network,
    window_size: Vec2,
    mouse_pos: Vec2,
    my_turn: bool,
}

impl App {
    pub fn new() -> Result<Self, ClientErr> {
        let mut app = Self {
            grid: None,
            menu: Menu::new(),
            network: Network::new()?,
            window_size: screen_size().into(),
            mouse_pos: Vec2::new(0.0, 0.0),
            my_turn: false,
        };

        app.menu.refresh_rooms(&mut app.network);

        Ok(app)
    }

    pub fn check_resize(&mut self) {
        let current_size: Vec2 = screen_size().into();

        let (width, height) = (screen_width(), screen_height());
        if width < MIN_WIDTH {
            let new_width = width.max(MIN_WIDTH);
            macroquad::prelude::request_new_screen_size(new_width, height);
        }

        if height < MIN_HEIGHT {
            let new_height = height.max(MIN_HEIGHT);
            macroquad::prelude::request_new_screen_size(width, new_height);
        }

        if self.window_size.x != current_size.x || self.window_size.y != current_size.y {
            if let Some(grid) = &mut self.grid {
                grid.center();
            }

            self.window_size = current_size;
        }
    }

    pub fn handle_input(&mut self) {
        self.menu.handle_input(&mut self.network);

        if let Some(grid) = &mut self.grid {
            let current_mouse_pos = mouse_position().into();

            if self.mouse_pos != current_mouse_pos {
                if let Some((i, j)) = grid.get_tile(current_mouse_pos) {
                    grid.highlight((i, j));
                } else {
                    grid.reset_highlight();
                }

                self.mouse_pos = current_mouse_pos;
            }

            if is_mouse_button_pressed(MouseButton::Left)
                && self.my_turn
                && let Some((y, x)) = grid.get_tile(current_mouse_pos)
                && let Err(e) = self.network.make_turn(y, x)
            {
                eprintln!("Error at make turn [{}]", e);
            }
        }
    }

    pub async fn update_state(&mut self) -> Result<(), ClientErr> {
        match self.network.check_for_updates() {
            Ok(Update::StartGame) => {
                self.network.get_opponent_username()?;
            }
            Ok(Update::YourTurn) => {
                self.network.request_tiles(&mut self.grid).await?;
                self.my_turn = true;
            }
            Ok(Update::WaitTurn) => {
                self.network.request_tiles(&mut self.grid).await?;
                self.my_turn = false;
            }
            Ok(Update::GameOver) => {
                self.network.read_tiles(&mut self.grid).await?;

                clear_background(Color::from_hex(0x3B4953));
                self.render();

                draw_text(
                    "GAME OVER!",
                    screen_width() / 3.5,
                    screen_height() / 2.0,
                    86.0,
                    Color::from_hex(0xF54927),
                );

                next_frame().await;

                thread::sleep(time::Duration::from_secs(5));

                self.menu.refresh_rooms(&mut self.network);
                self.menu.visible = true;
                self.network.room_id = None;
                self.grid = None;
                self.my_turn = false;
            }
            Ok(Update::None) => {}
            Err(_) => {}
        }

        Ok(())
    }

    pub fn render(&mut self) {
        self.menu.render();
        if let Some(grid) = &self.grid {
            grid.render();

            let text = if self.my_turn {
                "My Turn"
            } else {
                "Opponent's Turn"
            };
            let col_value = if self.my_turn { 0.9 } else { 0.1 };
            draw_text(
                text,
                screen_width() * 0.20,
                32.0,
                36.0,
                Color::new(1.0 - col_value, col_value, 0.1, 1.0),
            );

            draw_text(
                &format!(
                    "{} vs {}",
                    self.menu.username, &self.network.opponent_username
                ),
                screen_width() * 0.6,
                32.0,
                36.0,
                Color::from_hex(0xEBF4DD),
            );
        } else if self.network.room_id.is_some() {
            let title = "Waiting for players (1/2)";
            let title_width = macroquad::text::measure_text(title, None, 32, 1.0).width;
            let x = macroquad::window::screen_width() * 0.5 - title_width * 0.5;
            draw_text(title, x, 32.0, 32.0, Color::from_hex(0xEBF4DD));
        }
    }
}
