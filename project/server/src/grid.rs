use crate::room::TurnResult;
use rand::random_range;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Entity {
    None,
    Mouse,
    Wall,
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Entity>>,
    mouse_pos: (usize, usize),
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles: Vec<Vec<Entity>> = vec![vec![Entity::None; width]; height];

        let mouse_pos = (height / 2, width / 2);
        tiles[mouse_pos.0][mouse_pos.1] = Entity::Mouse;

        Self::generate_walls(&mut tiles, 5);

        Self {
            width,
            height,
            tiles,
            mouse_pos,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut tiles: Vec<u8> = Vec::new();

        bytes.extend((self.width as u32).to_le_bytes());
        bytes.extend((self.height as u32).to_le_bytes());

        for (y, line) in self.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if *tile != Entity::None {
                    tiles.push(y as u8);
                    tiles.push(x as u8);
                    tiles.push(*tile as u8);
                }
            }
        }

        bytes.extend((tiles.len() as u32 / 3).to_le_bytes());
        bytes.extend(tiles);

        bytes
    }

    fn generate_walls(tiles: &mut [Vec<Entity>], mut num_walls: u8) {
        let height = tiles.len();
        let width = tiles[0].len();
        while num_walls > 0 {
            let x = rand::random_range(0..width);
            let y = rand::random_range(0..height);

            if tiles[y][x] == Entity::None {
                tiles[y][x] = Entity::Wall;
                num_walls -= 1;
            }
        }
    }

    pub fn place(&mut self, y: &usize, x: &usize, entity: Entity) -> TurnResult {
        if self.tiles[*y][*x] == Entity::None {
            self.tiles[*y][*x] = entity;
            TurnResult::Good
        } else {
            TurnResult::Bad
        }
    }

    pub fn move_mouse_random(&mut self) -> TurnResult {
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
                && self.tiles[i as usize][j as usize] == Entity::None
            {
                valid.push((i as usize, j as usize));
            }
        }

        if !valid.is_empty() {
            let idx = random_range(0..valid.len());
            let (i, j) = valid[idx];
            self.tiles[mi][mj] = Entity::None;
            self.tiles[i][j] = Entity::Mouse;
            self.mouse_pos = (i, j);

            if i == 0 || i == self.height - 1 || j == 0 || j == self.width - 1 {
                TurnResult::GameOver
            } else {
                TurnResult::Good
            }
        } else {
            TurnResult::GameOver
        }
    }

    pub fn move_mouse(&mut self, y: &usize, x: &usize) -> TurnResult {
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
                && self.tiles[i as usize][j as usize] == Entity::None
            {
                valid.push((i as usize, j as usize));
            }
        }

        if !valid.is_empty() {
            for (i, j) in valid {
                if i == *y && j == *x {
                    self.tiles[mi][mj] = Entity::None;
                    self.tiles[i][j] = Entity::Mouse;
                    self.mouse_pos = (i, j);

                    return if i == 0 || i == self.height - 1 || j == 0 || j == self.width - 1 {
                        TurnResult::GameOver
                    } else {
                        TurnResult::Good
                    };
                }
            }

            TurnResult::Bad
        } else {
            TurnResult::GameOver
        }
    }
}
