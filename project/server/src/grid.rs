use std::collections::VecDeque;

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

    pub fn place_random(&mut self, entity: Entity) -> TurnResult {
        let (mi, mj) = self.mouse_pos;
        let valid = self.get_valid_neighbours(mi, mj);

        if !valid.is_empty() {
            let idx = random_range(0..valid.len());
            let (y, x) = valid[idx];

            self.tiles[y][x] = entity;
            TurnResult::Good
        } else {
            TurnResult::GameOver
        }
    }

    pub fn move_mouse(&mut self, y: &usize, x: &usize) -> TurnResult {
        let (mi, mj) = self.mouse_pos;
        let valid = self.get_valid_neighbours(mi, mj);

        if !valid.is_empty() {
            if let Some((i, j)) = valid.iter().find(|(i, j)| i == y && j == x) {

                self.tiles[mi][mj] = Entity::None;
                self.tiles[*i][*j] = Entity::Mouse;
                self.mouse_pos = (*i, *j);

                return if *i == 0 || *i == self.height - 1 || *j == 0 || *j == self.width - 1 {
                    TurnResult::GameOver
                } else {
                    TurnResult::Good
                };
            }

            TurnResult::Bad
        } else {
            TurnResult::GameOver
        }
    }

    pub fn move_mouse_random(&mut self) -> TurnResult {
        let (mi, mj) = self.mouse_pos;
        let valid = self.get_valid_neighbours(mi, mj);

        if !valid.is_empty() {
            let dist_map = self.get_distance_map();
            let best_move = valid.iter()
                .min_by_key(|&(i, j)| {
                    dist_map[*i][*j]
                });

            if let Some(&(i, j)) = best_move {
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
        } else {
            TurnResult::GameOver
        }
    }

    fn get_valid_neighbours(&self, pi: usize, pj: usize) -> Vec<(usize, usize)> {
        let neighbours: Vec<(i32, i32)> = if pj.is_multiple_of(2) {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1)]
        } else {
            vec![(-1, 0), (1, 0), (0, -1), (0, 1), (1, -1), (1, 1)]
        };

        neighbours.iter()
            .map(|(di, dj)| (pi as i32 + di, pj as i32 + dj))
            .filter(|&(i, j)|
                i >= 0
                && i < self.height as i32
                && j >= 0
                && j < self.width as i32
                && self.tiles[i as usize][j as usize] == Entity::None)
            .map(|(i, j)| (i as usize, j as usize))
            .collect()
    }

    fn get_distance_map(&self) -> Vec<Vec<i32>> {
        let mut dist_map = vec![vec![i32::MAX; self.width]; self.height];
        let mut queue = VecDeque::new();

        let mut add_tile = |i: usize, j: usize| {
            if self.tiles[i][j] == Entity::None {
                dist_map[i][j] = 0;
                queue.push_back((i, j));
            }
        };

        for i in 0..self.height {
            add_tile(i, 0);
            add_tile(i, self.width - 1);
        }
        for j in 0..self.width {
            add_tile(0, j);
            add_tile(self.height - 1, j);
        }

        while let Some((i, j)) = queue.pop_front() {
            for (ni, nj) in self.get_valid_neighbours(i, j) {
                if dist_map[ni][nj] == i32::MAX {
                    dist_map[ni][nj] = dist_map[i][j] + 1;
                    queue.push_back((ni, nj));
                }
            }
        }

        dist_map
    }
}
