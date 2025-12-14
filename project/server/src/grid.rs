// use macroquad::prelude::Vec2;
// use macroquad::{
//     color::Color,
//     prelude::rand,
//     shapes::draw_poly,
//     window::{screen_height, screen_width},
// };

type Vec2 = (f32, f32);
type Color = (u8, u8, u8, u8);

const DEFAULT_HEX_RADIUS: f32 = 32.0;
const HEX_OUTLINE_THINKNESS: f32 = 6.0;

#[derive(Clone, Copy, PartialEq)]
enum Entity {
    None,
    Mouse,
    Wall,
}

struct Tile {
    pos: Vec2,
    color: Color,
    highlight: bool,
    holder: Entity,
}

impl Tile {
    fn new(pos: Vec2, color: Color) -> Self {
        Self {
            pos,
            color,
            highlight: false,
            holder: Entity::None,
        }
    }

    fn is_empty(&self) -> bool {
        self.holder == Entity::None
    }
    fn set_holder(&mut self, entity: Entity) {
        self.holder = entity;
    }

    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn toggle_highlight(&mut self, value: bool) {
        self.highlight = value;
    }

    fn render(&self, _offset: Vec2) {
        // let pos = self.pos + offset;
        // let darker = Color::new(
        //     self.color.r - 0.1,
        //     self.color.g - 0.1,
        //     self.color.b - 0.1,
        //     1.0,
        // );
        // draw_poly(pos.x, pos.y, 6, DEFAULT_HEX_RADIUS, 90.0, darker);
        // draw_poly(
        //     pos.x,
        //     pos.y,
        //     6,
        //     DEFAULT_HEX_RADIUS - HEX_OUTLINE_THINKNESS,
        //     90.0,
        //     if self.highlight {
        //         Color::new(1.0, 0.5, 0.5, 1.0)
        //     } else {
        //         self.color
        //     },
        // );
        //
        // match self.holder {
        //     Entity::Mouse => draw_poly(pos.x, pos.y, 36, 16.0, 0.0, Color::new(1.0, 0.1, 0.1, 1.0)),
        //     Entity::Wall => draw_poly(pos.x, pos.y, 36, 16.0, 0.0, Color::new(0.1, 0.1, 1.0, 1.0)),
        //     Entity::None => {}
        // }
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    center: Vec2,
    mouse_pos: (usize, usize),
    highlighted: Option<(usize, usize)>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let hex_width = DEFAULT_HEX_RADIUS * 3.0_f32.sqrt();
        let hex_height = DEFAULT_HEX_RADIUS * 1.5;

        let center: Vec2 = (500.0, 500.0);
        // let center = Vec2::from((screen_width(), screen_height())) / 2.0
            // - Self::get_grid_size(width, height) / 2.0;

        let (cx, cy) = (width as f32 / 2.0, height as f32 / 2.0);
        let mut tiles: Vec<Vec<Tile>> = (0..height)
            .map(|i| {
                (0..width)
                    .map(|j| {
                        let _diff = ((cx - j as f32).abs() + (cy - i as f32).abs()) / 16.0;
                        // let base_color = Color::new(0.8 - diff, 0.8, 0.8 - diff, 1.0);

                        let base_color = (32, 32, 32, 255);
                        Tile::new(
                            (
                                i as f32 * hex_width
                                    + if j % 2 == 1 { hex_width / 2.0 } else { 0.0 },
                                j as f32 * hex_height,
                            ),
                            base_color,
                        )
                    })
                    .collect()
            })
            .collect();

        let mouse_pos = (height / 2, width / 2);
        tiles[mouse_pos.0][mouse_pos.1].set_holder(Entity::Mouse);

        Self::generate_walls(&mut tiles, 5);

        Self {
            width,
            height,
            center,
            mouse_pos,
            highlighted: None,
            tiles,
        }
    }

    pub fn center(&mut self) {
        // self.center = Vec2::from((screen_width(), screen_height())) / 2.0
        //     - Self::get_grid_size(self.width, self.height) / 2.0;
    }

    fn get_grid_size(width: usize, height: usize) -> Vec2 {
        let hex_width = DEFAULT_HEX_RADIUS * 3.0_f32.sqrt();
        let hex_height = DEFAULT_HEX_RADIUS * 1.5;

        (
            hex_width * width as f32 - hex_width * 0.5,
            hex_height * (height - 1) as f32,
        )
    }

    fn generate_walls(_tiles: &mut [Vec<Tile>], mut _num_walls: u8) {
        // let height = tiles.len();
        // let width = tiles[0].len();
        // while num_walls > 0 {
        //     let x = rand::gen_range(0, width);
        //     let y = rand::gen_range(0, height);
        //
        //     if tiles[y][x].is_empty() {
        //         tiles[y][x].set_holder(Entity::Wall);
        //         num_walls -= 1;
        //     }
        // }
    }

    pub fn get_tile(&self, mut _pos: Vec2) -> Option<(usize, usize)> {
        // pos -= self.center;
        //
        // let hex_width = DEFAULT_HEX_RADIUS * 3.0_f32.sqrt();
        // let hex_height = DEFAULT_HEX_RADIUS * 1.5;
        //
        // let j = (pos.y / hex_height).round() as i32;
        // if j < 0 || j as usize >= self.height {
        //     return None;
        // }
        //
        // let offset = if j % 2 == 1 { hex_width * 0.5 } else { 0.0 };
        // let i = ((pos.x - offset) / hex_width).round() as i32;
        // if i < 0 || i as usize >= self.width {
        //     return None;
        // }
        //
        // let i = i as usize;
        // let j = j as usize;
        //
        // let pos = pos - self.tiles[i][j].get_pos();
        // if pos.x.abs() > hex_width / 2.0 {
        //     return None;
        // }
        //
        // if pos.y.abs() > DEFAULT_HEX_RADIUS {
        //     return None;
        // }
        //
        // if pos.y.abs() * (hex_width / 2.0 / DEFAULT_HEX_RADIUS) + pos.x.abs() > hex_width / 2.0 {
        //     return None;
        // }
        //
        // Some((i, j))
        None
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

    pub fn place_wall(&mut self) -> bool {
        if let Some((i, j)) = self.highlighted
            && self.tiles[i][j].is_empty()
        {
            self.tiles[i][j].set_holder(Entity::Wall);
            println!("Cliecked {i} {j}");

            return true;
        }

        false
    }

    pub fn move_mouse(&mut self) {
        let (mi, mj) = self.mouse_pos;
        let neighbours: Vec<(i32, i32)> = if mj % 2 == 0 {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1)]
        } else {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1), (1, -1), (1, 1)]
        };
        let mut valid = Vec::<(usize, usize)>::new();

        for (di, dj) in neighbours.iter() {
            let i = mi as i32 + di;
            let j = mj as i32 + dj;

            if i >= 0
                && i < self.height as i32
                && j >= 0
                && j < self.width as i32
                && self.tiles[i as usize][j as usize].is_empty()
            {
                valid.push((i as usize, j as usize));
            }
        }

        if !valid.is_empty() {
            // let idx = rand::gen_range(0, valid.len());
            // let (i, j) = valid[idx];
            // self.tiles[i][j].set_holder(Entity::Mouse);
            // self.tiles[mi][mj].set_holder(Entity::None);
            // self.mouse_pos = (i, j);
        } else {
            todo!("Soarecele a pierdut!");
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
