use crate::grid::{Entity, Grid};
use uuid::Uuid;

pub enum TurnResult {
    Good,
    Bad,
    GameOver,
}

pub struct Room {
    pub players: Vec<Uuid>,
    max_players: u8,
    grid: Grid,
}

impl Room {
    pub fn new(max_players: u8) -> Self {
        Self {
            players: Vec::new(),
            max_players,
            grid: Grid::new(11, 11),
        }
    }

    pub fn is_available(&self) -> bool {
        self.get_player_count() < self.max_players
    }

    pub fn add_player(&mut self, uid: &Uuid) {
        self.players.push(*uid);
    }

    pub fn get_player_count(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn get_grid(&self) -> Vec<u8> {
        self.grid.as_bytes()
    }

    pub fn get_other_player(&self, uid: &Uuid) -> Option<Uuid> {
        if self.get_player_count() == 2 {
            if self.players[0] == *uid {
                Some(self.players[1])
            } else {
                Some(self.players[0])
            }
        } else {
            None
        }
    }

    pub fn is_user_in(&self, uid: &Uuid) -> bool {
        for user in &self.players {
            if *user == *uid {
                return true;
            }
        }

        false
    }

    pub fn process_turn(&mut self, uid: &Uuid, y: &usize, x: &usize) -> TurnResult {
        if self.players[0] == *uid {
            self.grid.place(y, x, Entity::Wall)
        } else {
            self.grid.move_mouse(y, x)
        }
    }
}
