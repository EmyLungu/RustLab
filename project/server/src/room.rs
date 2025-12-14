use uuid::Uuid;
use crate::grid::Grid;

pub struct Room {
    players: Vec<Uuid>,
    grid: Grid,
}

impl Room {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            grid: Grid::new(11, 11),
        }
    }

    pub fn add_player(&mut self, uid: &Uuid) {
        self.players.push(*uid);
    }

    pub fn get_player_count(&self) -> u8 {
        self.players.len() as u8
    }
}
