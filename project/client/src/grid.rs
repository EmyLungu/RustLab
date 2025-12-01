use macroquad::{color::Color, shapes::draw_poly, window::{screen_height, screen_width}};
use macroquad::prelude::Vec2;

const DEFAULT_HEX_RADIUS: f32 = 32.0;
const HEX_OUTLINE_THINKNESS: f32 = 6.0;

struct Tile {
    pos: Vec2,
    color: Color,
    highlight: bool
}

impl Tile {
    fn new(pos: Vec2, color: Color) -> Self {
        Self {
            pos,
            color,
            highlight: false
        }
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn toggle_highlight(&mut self, value: bool) {
        self.highlight = value;
    }

    fn render(&self, offset: Vec2) {
        let darker = Color::new(self.color.r - 0.1, self.color.g - 0.1, self.color.b - 0.1, 1.0);
        draw_poly(self.pos.x + offset.x, self.pos.y + offset.y, 6, DEFAULT_HEX_RADIUS, 90.0, darker);
        draw_poly(
            self.pos.x + offset.x,
            self.pos.y + offset.y,
            6,
            DEFAULT_HEX_RADIUS - HEX_OUTLINE_THINKNESS,
            90.0,
            if self.highlight { Color::new(1.0, 0.5, 0.5, 1.0) } else { self.color }
        );
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    center: Vec2,
    highlighted: Option<(usize, usize)>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let hex_width = DEFAULT_HEX_RADIUS * 3.0_f32.sqrt();
        let hex_height = DEFAULT_HEX_RADIUS * 1.5;

        let center = Vec2::from((screen_width(), screen_height())) / 2.0 - Self::get_grid_size(width, height) / 2.0;

        let (cx, cy) = (width as f32 / 2.0, height as f32 / 2.0);
        Self {
            width,
            height,
            center,
            highlighted: None,
            tiles: (0..height)
                .map(|i| {
                    (0..width).map(|j| {
                        let diff = ((cx - j as f32).abs() + (cy - i as f32).abs()) / 16.0;
                        let base_color = Color::new(0.8 - diff, 0.8, 0.8 - diff, 1.0);

                        Tile::new(
                            Vec2::new(
                                i as f32 * hex_width + if j % 2 == 1 { hex_width / 2.0 } else { 0.0 },
                                j as f32 * hex_height
                            ),
                            base_color
                        )
                    }).collect()
                }).collect()
        }
    }

    pub fn center(&mut self) {
        self.center = Vec2::from((screen_width(), screen_height())) / 2.0 -
            Self::get_grid_size(self.width, self.height) / 2.0;
    }

    fn get_grid_size(width: usize, height: usize) -> Vec2 {
        let hex_width = DEFAULT_HEX_RADIUS * 3.0_f32.sqrt();
        let hex_height = DEFAULT_HEX_RADIUS * 1.5;

        Vec2::new(
            hex_width * width as f32 - hex_width * 0.5,
            hex_height * (height - 1) as f32, 
        )
    }

    pub fn get_tile(&self, mut pos: Vec2) -> Option<(usize, usize)> {
        pos -= self.center;

        let hex_width = DEFAULT_HEX_RADIUS * 3.0_f32.sqrt();
        let hex_height = DEFAULT_HEX_RADIUS * 1.5;

        let j = (pos.y / hex_height).round() as i32;
        if j < 0 || j as usize >= self.height {
            return None;
        }

        let offset = if j % 2 == 1 { hex_width * 0.5 } else { 0.0 };
        let i = ((pos.x - offset) / hex_width).round() as i32;
        if i < 0 || i as usize >= self.width {
            return None;
        }

        let i = i as usize;
        let j = j as usize;

        let pos = pos - self.tiles[i][j].get_pos();
        if pos.x.abs() > hex_width / 2.0 {
            return None;
        }

        if pos.y.abs() > DEFAULT_HEX_RADIUS {
            return None;
        }

        if pos.y.abs() * (hex_width / 2.0 / DEFAULT_HEX_RADIUS ) + pos.x.abs() > hex_width / 2.0 {
            return None;
        }

        Some((i, j))
    }

    pub fn highlight(&mut self, (i, j): (usize, usize)) {
        if self.highlighted.is_some() {
            self.reset_highlight();
        }
        self.highlighted = Some((i, j));
        self.tiles[i][j].toggle_highlight(true);
    }

    pub fn reset_highlight(&mut self) {
        if let Some((i, j)) = self.highlighted {
            self.tiles[i][j].toggle_highlight(false);
            self.highlighted = None;
        }
    }

    pub fn render(&self) {
        for line in self.tiles.iter() {
            for tile in line {
                tile.render(self.center);
            }
        }
    }
}

