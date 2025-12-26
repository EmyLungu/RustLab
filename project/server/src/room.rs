use crate::grid::{Entity, Grid};
use uuid::Uuid;

#[derive(PartialEq)]
pub enum TurnResult {
    Good,
    Bad,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerType {
    Mouse,
    Wall,
}

pub struct Room {
    pub players: Vec<(Uuid, PlayerType)>,
    pub max_players: u8,
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

    pub fn add_player(&mut self, uid: &Uuid, player_type: &PlayerType) {
        self.players.push((*uid, *player_type));
    }

    fn get_player_type(&self, uid: &Uuid) -> Option<PlayerType> {
        self.players
            .iter()
            .find(|(id, _)| *id == *uid)
            .map(|(_, ptype)| *ptype)
    }

    pub fn get_player_count(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn get_grid(&self) -> Vec<u8> {
        self.grid.as_bytes()
    }

    pub fn get_other_player(&self, uid: &Uuid) -> Option<Uuid> {
        if self.get_player_count() == 2 {
            if self.players[0].0 == *uid {
                Some(self.players[1].0)
            } else {
                Some(self.players[0].0)
            }
        } else {
            None
        }
    }

    pub fn process_turn(&mut self, uid: &Uuid, y: &usize, x: &usize) -> TurnResult {
        if let Some(player_type) = self.get_player_type(uid) {
            match player_type {
                PlayerType::Mouse => self.grid.move_mouse(y, x),
                PlayerType::Wall => self.grid.place(y, x, Entity::Wall),
            }
        } else {
            TurnResult::Bad
        }
    }

    pub fn ai_turn(&mut self) -> TurnResult {
        match self.players[0].1 {
            PlayerType::Mouse => self.grid.place_random(Entity::Wall),
            PlayerType::Wall => self.grid.move_mouse_random(),
        }
    }
}
