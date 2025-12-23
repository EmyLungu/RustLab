use macroquad::{
    color::Color, input::{
        KeyCode, MouseButton, get_char_pressed, is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position
    }, prelude::Vec2, shapes::{draw_poly, draw_rectangle}, text::{draw_text, measure_text}, texture::{Texture2D, draw_texture, load_texture}, window::{screen_height, screen_width}
};

use crate::network::Network;
use crate::button::{Button, ButtonType};

const MENU_OFFSET: f32 = 64.0;
const START_ROOMS_Y: f32 = 286.0;
const ROOM_HEIGHT: f32 = 64.0;

const MAX_USERNAME: usize = 10;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PlayerType {
    Mouse,
    Wall,
}

struct Room {
    room_id: [u8; 16],
    button: Button,
}

pub struct Menu {
    pub visible: bool,
    pub username: String,
    writing_mode: bool,
    player_type: PlayerType,
    rooms: Vec<Room>,
    buttons: [Button; 5],
    mouse_tex: Option<Texture2D>,
}

impl Menu {
    pub fn new() -> Self {
        let buttons = [
            Button::new(
                ButtonType::Refresh,
                Vec2::new(-7.0 * MENU_OFFSET - 32.0, START_ROOMS_Y - 64.0 + 8.0),
                Vec2::new(124.0, ROOM_HEIGHT - 32.0),
                "Refresh".to_string(),
                Color::from_hex(0x6498D99),
                true,
            ),
            Button::new(
                ButtonType::StartGameBot,
                Vec2::new(-4.5 * MENU_OFFSET - 32.0, START_ROOMS_Y - 64.0 + 8.0),
                Vec2::new(216.0, ROOM_HEIGHT - 32.0),
                "New Game (BOT)".to_string(),
                Color::from_hex(0xB07F23),
                true,
            ),
            Button::new(
                ButtonType::EnterText,
                Vec2::new(MENU_OFFSET + 32.0, START_ROOMS_Y - 128.0),
                Vec2::new(300.0, ROOM_HEIGHT - 32.0),
                "Guest".to_string(),
                Color::from_hex(0xB07F23),
                false,
            ),
            Button::new(
                ButtonType::LeftSelect,
                Vec2::new(MENU_OFFSET + 8.0, START_ROOMS_Y + 128.0),
                Vec2::new(56.0, 56.0),
                "<".to_string(),
                Color::from_hex(0x6498D99),
                false,
            ),
            Button::new(
                ButtonType::RightSelect,
                Vec2::new(MENU_OFFSET + 144.0, START_ROOMS_Y + 128.0),
                Vec2::new(56.0, 56.0),
                ">".to_string(),
                Color::from_hex(0x6498D99),
                false,
            ),
        ];

        Self {
            visible: true,
            writing_mode: false,
            username: String::from("Guest"),
            player_type: PlayerType::Mouse,
            rooms: Vec::new(),
            buttons,
            mouse_tex: None,
        }
    }

    pub async fn load_textures(&mut self) {
        self.mouse_tex = load_texture("assets/mouse.png").await.ok();
    }

    pub fn refresh_rooms(&mut self, network: &mut Network) {
        self.rooms.clear();

        match network.request_rooms() {
            Ok(rooms) => {
                for (idx, (room_id, player_count)) in rooms.iter().enumerate() {
                    self.rooms.push(Room {
                        room_id: *room_id,
                        button: Button::new(
                            ButtonType::Room,
                            Vec2::new(-8.0 * MENU_OFFSET, START_ROOMS_Y + (ROOM_HEIGHT + 8.0) * idx as f32),
                            Vec2::new(400.0, ROOM_HEIGHT),
                            format!("\t\t\tRoom {}\t({}/2) players", idx + 1, player_count),
                            Color::from_hex(0x5A7863),
                            true
                        )
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

        if self.writing_mode {
            self.handle_writing();
            return;
        }

        if is_key_down(KeyCode::RightControl) && is_key_pressed(KeyCode::R) {
            self.refresh_rooms(network);
        }

        let mouse_pos: Vec2 = mouse_position().into();
        for button in self.buttons.iter_mut() {
            button.highlighted = button.is_inside(mouse_pos);
        }
        for room in self.rooms.iter_mut() {
            room.button.highlighted = room.button.is_inside(mouse_pos);
        }

        if is_mouse_button_pressed(MouseButton::Left) {

            let mut clicked: Option<ButtonType> = None;
            let mut room_id: Option<[u8; 16]> = None;
            for button in &self.buttons {
                if button.is_inside(mouse_pos) {
                    clicked = Some(button.button_type);
                    break;
                }
            }

            if clicked.is_none() {
                for room in &self.rooms {
                    if room.button.is_inside(mouse_pos) {
                        clicked = Some(room.button.button_type);
                        room_id = Some(room.room_id);
                        break;
                    }
                }
            }

            if let Some(button_type) = clicked {
                self.handle_button(button_type, room_id, network);
            }
        }
    }

    pub fn handle_writing(&mut self) {
        let mut update = false;

        while let Some(c) = get_char_pressed() {
            if !c.is_control() && self.username.chars().count() < MAX_USERNAME {
                self.username.push(c);
                update = true;
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.username.pop();
            update = true;
        }

        if is_key_pressed(KeyCode::Enter) {
            self.writing_mode = false;
            if let Some(b) = &mut self.buttons
                .iter_mut()
                .find(|b| b.button_type == ButtonType::EnterText)
            {
                b.color = Color::from_hex(0xB07F23);
            }
        }

        if update &&
            let Some(b) = &mut self.buttons
                .iter_mut()
                .find(|b| b.button_type == ButtonType::EnterText)
        {
            b.text = self.username.clone();
        }
    }

    fn handle_button(
        &mut self,
        button_type: ButtonType,
        room_id: Option<[u8; 16]>,
        network: &mut Network)
    {
        match button_type {
            ButtonType::Refresh => self.refresh_rooms(network),
            ButtonType::StartGameBot => {
                match network.start_room_bot(&self.player_type, &self.username) {
                    Ok(()) => self.visible = false,
                    Err(e) => eprintln!("Could not start new bot room ({})", e),
                }
            },
            ButtonType::EnterText => self.writing_mode = true,
            ButtonType::LeftSelect => self.swap_player_type(),
            ButtonType::RightSelect => self.swap_player_type(),
            ButtonType::Room => {
                if let Some(rid) = room_id {
                    match network.join_room(&rid, &self.player_type, &self.username) {
                        Ok(()) => self.visible = false,
                        Err(e) => eprintln!("Could not send join room request ({})", e),
                    }
                }
            }
        }
    }

    fn swap_player_type(&mut self) {
        self.player_type = match self.player_type {
            PlayerType::Mouse => PlayerType::Wall,
            PlayerType::Wall => PlayerType::Mouse,
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
            MENU_OFFSET + 48.0,
            64.0,
            Color::from_hex(0xEBF4DD),
        );

        draw_text(
            "\t\t\tEnter your username:",
            MENU_OFFSET,
            START_ROOMS_Y - 132.0,
            32.0,
            Color::from_hex(0xEBF4DD),
        );

        if let Some(tex) = &self.mouse_tex {
            let text = match self.player_type {
                PlayerType::Mouse => {
                    draw_texture(
                        tex, MENU_OFFSET + 64.0, START_ROOMS_Y + 64.0,
                        Color::from_hex(0xFFFFFF)
                    );

                    "Mouse"
                }
                PlayerType::Wall => {
                    draw_poly(
                        MENU_OFFSET + 104.0,
                        START_ROOMS_Y + 86.0,
                        6, 48.0, 90.0,
                        Color::from_hex(0x964B00)
                    );

                    "Wall"
                },
            };


            draw_text(
                &("Play as: ".to_owned() + text),
                MENU_OFFSET + 32.0,
                START_ROOMS_Y + 24.0,
                32.0,
                Color::from_hex(0xEBF4DD),
            );
        }

        draw_rectangle(
            MENU_OFFSET + 16.0,
            START_ROOMS_Y - 64.0,
            screen_width() - 2.0 * MENU_OFFSET - 32.0,
            ROOM_HEIGHT - 16.0,
            Color::from_hex(0x5A7863),
        );

        draw_text(
            "\t\t\tRooms:",
            MENU_OFFSET,
            START_ROOMS_Y - 64.0 + ROOM_HEIGHT * 0.5,
            32.0,
            Color::from_hex(0xEBF4DD),
        );

        for button in &self.buttons {
            button.render();
        }

        for room in self.rooms.iter() {
            room.button.render();
        }
    }
}
