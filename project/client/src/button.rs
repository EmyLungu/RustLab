use macroquad::{
    color::Color,
    prelude::Vec2,
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    window::screen_width,
};

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonType {
    Refresh,
    StartGameBot,
    EnterText,
    LeftSelect,
    RightSelect,
    Room,
}

pub struct Button {
    pub button_type: ButtonType,
    pub highlighted: bool,
    pub color: Color,
    pub text: String,
    pos: Vec2,
    size: Vec2,
    responsive: bool,
}

impl Button {
    pub fn new(
        button_type: ButtonType,
        pos: Vec2,
        size: Vec2,
        text: String,
        color: Color,
        responsive: bool,
    ) -> Self {
        Self {
            button_type,
            pos,
            size,
            text,
            color,
            responsive,
            highlighted: false,
        }
    }

    pub fn is_inside(&self, p: Vec2) -> bool {
        let offset = if self.responsive {
            Vec2::new(screen_width(), 0.0)
        } else {
            Vec2::new(0.0, 0.0)
        };

        let pos = self.pos + offset;

        p.x > pos.x && p.x < pos.x + self.size.x && p.y > pos.y && p.y < pos.y + self.size.y
    }

    pub fn render(&self) {
        let offset = if self.responsive {
            Vec2::new(screen_width(), 0.0)
        } else {
            Vec2::new(0.0, 0.0)
        };

        let pos = self.pos + offset;
        draw_rectangle(
            pos.x,
            pos.y,
            self.size.x,
            self.size.y,
            if self.highlighted {
                Color::new(
                    self.color.r * 0.9,
                    self.color.g * 0.9,
                    self.color.b * 0.9,
                    self.color.a,
                )
            } else {
                self.color
            },
        );

        let dim = measure_text(&self.text, None, 32, 1.0);
        let center = pos + self.size * 0.5;
        draw_text(
            &self.text,
            center.x - (dim.width * 0.5),
            center.y + (dim.offset_y * 0.5),
            32.0,
            Color::from_hex(0xEBF4DD),
        );
    }
}
