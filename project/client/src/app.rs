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
};

pub struct App {
    grid: Option<Grid>,
    menu: Menu,
    network: Network,
    window_size: Vec2,
    mouse_pos: Vec2,
    my_turn: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            grid: None,
            menu: Menu::new(),
            network: Network::new(),
            window_size: screen_size().into(),
            mouse_pos: Vec2::new(0.0, 0.0),
            my_turn: false,
        }
    }

    pub fn check_resize(&mut self) {
        let current_size: Vec2 = screen_size().into();

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
                eprintln!("Errror at make turn [{}]", e);
            }
        }
    }

    pub fn update_state(&mut self) -> Result<(), std::io::Error> {
        match self.network.check_for_updates() {
            Ok(Update::YourTurn) => {
                self.network.request_tiles(&mut self.grid)?;
                self.my_turn = true;
            }
            Ok(Update::WaitTurn) => {
                self.network.request_tiles(&mut self.grid)?;
                self.my_turn = false;
            }
            Ok(Update::GameOver) => {
                self.menu.refresh_rooms(&mut self.network);
                self.menu.visible = true;
                self.network.room_id = None;
                self.grid = None;
                self.my_turn = false;
                println!("Finished game over!");
            }
            Ok(Update::None) => {}
            Err(_) => {}
        }

        Ok(())
    }

    pub fn render(&self) {
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
                32.0,
                64.0,
                36.0,
                Color::new(1.0 - col_value, col_value, 0.1, 1.0),
            );
        }
    }
}
