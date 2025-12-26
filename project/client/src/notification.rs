use macroquad::{
    color::Color,
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    window::screen_width,
};

const SUCCESS_COLOR: Color = Color::from_hex(0x007E6E);
const ERROR_COLOR: Color = Color::from_hex(0xBB3E00);

struct Notification {
    life: u32,
    width: f32,
    height: f32,
    y: f32,
    text: String,
    color: Color,
}

impl Notification {
    fn new(message: String, color: Color, i: f32) -> Self {
        Self {
            life: 256,
            width: message.chars().count() as f32 * 16.0,
            height: 32.0,
            y: 32.0 + i * 64.0,
            text: message,
            color,
        }
    }
}

impl Notification {
    fn render(&self) {
        let (x, y) = (screen_width() - self.width - 16.0, self.y);

        let text_dims = measure_text(&self.text, None, 32, 1.0);
        let text_x = x + self.width / 2.0 - text_dims.width / 2.0;
        let text_y = y + self.height / 2.0 + text_dims.height / 2.0;

        draw_rectangle(x, y, self.width, self.height, self.color);
        draw_text(&self.text, text_x, text_y, 32.0, Color::from_hex(0xEBF4DD));
    }
}

pub struct NotificaitonsManager {
    notifications: Vec<Notification>,
}

impl NotificaitonsManager {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }

    pub fn add(&mut self, message: String, is_success: bool) {
        self.notifications.push(Notification::new(
            message,
            if is_success {
                SUCCESS_COLOR
            } else {
                ERROR_COLOR
            },
            self.notifications.len() as f32,
        ));
    }

    pub fn render(&mut self) {
        self.notifications.retain_mut(|notif| {
            notif.life -= 1;
            notif.life > 0
        });

        for notif in &self.notifications {
            notif.render();
        }
    }
}
