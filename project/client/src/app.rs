use crate::{menu::Menu, network::Network};

use macroquad::{
    input::{is_mouse_button_pressed, mouse_position, is_key_down, is_key_pressed, KeyCode},
    math::Vec2,
    miniquad::window::screen_size,
};

pub struct App {
    // grid: Grid,
    menu: Menu,
    network: Network,
    window_size: Vec2,
    mouse_pos: Vec2,
}

impl App {
    pub fn new() -> Self {
        Self {
            // grid: Grid::new(11, 11),
            menu: Menu::new(),
            network: Network::new(),
            window_size: screen_size().into(),
            mouse_pos: Vec2::new(0.0, 0.0),
        }
    }

    pub fn check_resize(&mut self) {
        let current_size: Vec2 = screen_size().into();

        if self.window_size.x != current_size.x || self.window_size.y != current_size.y {
            // self.grid.center();

            self.window_size = current_size;
        }
    }

    pub fn handle_input(&mut self) {
        self.menu.handle_input(&mut self.network);
        // let current_mouse_pos = mouse_position().into();
        // if self.mouse_pos != current_mouse_pos {
        //     if let Some((i, j)) = self.grid.get_tile(current_mouse_pos) {
        //         self.grid.highlight((i, j));
        //     } else {
        //         self.grid.reset_highlight();
        //     }
        //
        //     self.mouse_pos = current_mouse_pos;
        // }
        //
        // if is_mouse_button_pressed(macroquad::input::MouseButton::Left) && self.grid.place_wall() {
        //     self.grid.move_mouse();
        // }
    }

    pub fn render(&self) {
        self.menu.render();
        // self.grid.render();
    }
}
