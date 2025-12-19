use macroquad::{
    color::Color,
    input::{
        KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position,
    },
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::network::Network;

const MENU_OFFSET: f32 = 64.0;
const START_ROOMS_Y: f32 = 248.0;
const ROOM_HEIGHT: f32 = 64.0;

struct Room {
    room_id: [u8; 16],
    player_count: u8,
}

pub struct Menu {
    pub visible: bool,
    rooms: Vec<Room>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            visible: true,
            rooms: Vec::new(),
        }
    }

    pub fn refresh_rooms(&mut self, network: &mut Network) {
        self.rooms.clear();

        match network.request_rooms() {
            Ok(rooms) => {
                for (room_id, player_count) in rooms.iter() {
                    self.rooms.push(Room {
                        room_id: *room_id,
                        player_count: *player_count,
                    });
                }
            }
            Err(e) => eprintln!("Could not request rooms ({})", e),
        }
    }

    pub fn handle_input(&mut self, network: &mut Network) {
        if !self.visible {
            return;
        }

        if is_key_down(KeyCode::RightControl) && is_key_pressed(KeyCode::R) {
            self.refresh_rooms(network);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            if x > MENU_OFFSET && x < screen_width() - MENU_OFFSET && y > START_ROOMS_Y {
                let idx = ((y - START_ROOMS_Y) / (ROOM_HEIGHT + 8.0)) as usize;

                if idx < self.rooms.len() {
                    match network.join_room(&self.rooms[idx].room_id, "eeemy".to_string()) {
                        Ok(()) => self.visible = false,
                        Err(e) => eprintln!("Could not send join room request ({})", e),
                    }
                }
            }
        }
    }

    pub fn render(&self) {
        if !self.visible {
            return;
        }

        draw_rectangle(
            MENU_OFFSET,
            MENU_OFFSET,
            screen_width() - 2.0 * MENU_OFFSET,
            screen_height() - 2.0 * MENU_OFFSET,
            Color::from_hex(0x90AB8B),
        );

        let title = "Trap the mouse";
        let title_width = measure_text(title, None, 64, 1.0).width;
        let x = screen_width() * 0.5 - title_width * 0.5;
        draw_text(
            title,
            x,
            MENU_OFFSET + 64.0,
            64.0,
            Color::from_hex(0xEBF4DD),
        );

        let mut room_y = START_ROOMS_Y;
        for (idx, room) in self.rooms.iter().enumerate() {
            draw_rectangle(
                MENU_OFFSET + 16.0,
                room_y,
                screen_width() - 2.0 * MENU_OFFSET - 32.0,
                ROOM_HEIGHT,
                Color::from_hex(0x5A7863),
            );

            draw_text(
                format!("\t\t\tRoom {}\t({}/2) players", idx + 1, room.player_count).as_str(),
                MENU_OFFSET,
                room_y + ROOM_HEIGHT * 0.5,
                32.0,
                Color::from_hex(0xEBF4DD),
            );

            room_y += ROOM_HEIGHT + 8.0;
        }
    }
}
