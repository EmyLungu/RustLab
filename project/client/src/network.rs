use crate::app::ClientErr;
use crate::grid::Grid;
use crate::menu::PlayerType;
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

#[repr(u8)]
pub enum Protocol {
    RequestRooms,
    StartRoomBot,
    JoinRoom,
    JoinSuccess,
    JoinFail,
    RequestTiles,
    StartGame,
    Turn,
    WaitTurn,
    YourTurn,
    GameOver,
}

pub enum Update {
    None,
    StartGame,
    YourTurn,
    WaitTurn,
    GameOver,
}

type RoomId = [u8; 16];
type RoomData = Vec<(RoomId, u8)>;

pub struct Network {
    stream: TcpStream,
    pub room_id: Option<RoomId>,
    pub opponent_username: String,
}

impl Network {
    pub fn new() -> Result<Self, ClientErr> {
        if let Ok(mut addrs) = "127.0.0.1:1922".to_socket_addrs()
            && let Some(addr) = addrs.next()
        {
            let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5))?;
            Ok(Self {
                stream,
                room_id: None,
                opponent_username: String::new(),
            })
        } else {
            Err(ClientErr::JoinFail)
        }
    }

    pub fn request_rooms(&mut self) -> Result<RoomData, ClientErr> {
        self.stream.write_all(&[Protocol::RequestRooms as u8])?;

        let mut room_countb = [0u8; 4];
        self.stream.read_exact(&mut room_countb)?;

        let room_count = u32::from_le_bytes(room_countb);
        let mut room_data: RoomData = Vec::new();

        for _ in 0..room_count {
            let mut room_idb = [0u8; 16];
            self.stream.read_exact(&mut room_idb)?;

            let mut player_countb = [0u8; 1];
            self.stream.read_exact(&mut player_countb)?;

            let player_count = u8::from_le_bytes(player_countb);
            room_data.push((room_idb, player_count));
        }

        Ok(room_data)
    }

    pub fn start_room_bot(
        &mut self,
        player_type: &PlayerType,
        username: &str,
    ) -> Result<(), ClientErr> {
        if username.chars().count() == 0 {
            return Err(ClientErr::InvalidUsername);
        }

        self.stream.write_all(&[Protocol::StartRoomBot as u8])?;
        self.stream.write_all(&[*player_type as u8])?;
        self.stream
            .write_all(&(username.len() as u32).to_le_bytes())?;
        self.stream.write_all(username.as_bytes())?;

        let mut responseb = [0u8; 1];
        self.stream.read_exact(&mut responseb)?;

        if responseb[0] == Protocol::JoinSuccess as u8 {
            let mut room_idb = [0u8; 16];
            self.stream.read_exact(&mut room_idb)?;

            self.room_id = Some(room_idb);
        }

        Ok(())
    }

    pub fn join_room(
        &mut self,
        room_id: &RoomId,
        player_type: &PlayerType,
        username: &str,
    ) -> Result<(), ClientErr> {
        if username.chars().count() == 0 {
            return Err(ClientErr::InvalidUsername);
        }

        self.stream.write_all(&[Protocol::JoinRoom as u8])?;
        self.stream.write_all(room_id)?;
        self.stream.write_all(&[*player_type as u8])?;
        self.stream
            .write_all(&(username.len() as u32).to_le_bytes())?;
        self.stream.write_all(username.as_bytes())?;

        let mut responseb = [0u8; 1];
        self.stream.read_exact(&mut responseb)?;

        if responseb[0] == Protocol::JoinSuccess as u8 {
            self.room_id = Some(*room_id);
        } else if responseb[0] == Protocol::JoinFail as u8 {
            return Err(ClientErr::JoinFail);
        }

        Ok(())
    }

    pub async fn request_tiles(&mut self, grid: &mut Option<Grid>) -> Result<(), ClientErr> {
        if let Some(room_id) = self.room_id {
            self.stream.write_all(&[Protocol::RequestTiles as u8])?;
            self.stream.write_all(&room_id)?;

            self.read_tiles(grid).await?;
        }
        Ok(())
    }

    pub async fn read_tiles(&mut self, grid: &mut Option<Grid>) -> Result<(), ClientErr> {
        let mut sizeb = [0u8; 4];
        self.stream.read_exact(&mut sizeb)?;
        let width = u32::from_le_bytes(sizeb) as usize;

        self.stream.read_exact(&mut sizeb)?;
        let height = u32::from_le_bytes(sizeb) as usize;

        self.stream.read_exact(&mut sizeb)?;
        let tile_count = u32::from_le_bytes(sizeb) as usize;

        let mut new_grid = Grid::new(width, height);
        let mut byte = [0u8; 1];

        for _ in 0..tile_count {
            self.stream.read_exact(&mut byte)?;
            let y = byte[0] as usize;

            self.stream.read_exact(&mut byte)?;
            let x = byte[0] as usize;

            self.stream.read_exact(&mut byte)?;
            let entity = byte[0];

            new_grid.place_entity(y, x, entity);
        }

        new_grid.load_textures().await;
        *grid = Some(new_grid);
        Ok(())
    }

    pub fn make_turn(&mut self, y: usize, x: usize) -> Result<(), ClientErr> {
        self.stream.write_all(&[Protocol::Turn as u8])?;
        self.stream.write_all(&(y as u32).to_le_bytes())?;
        self.stream.write_all(&(x as u32).to_le_bytes())?;

        Ok(())
    }

    pub fn get_opponent_username(&mut self) -> Result<(), ClientErr> {
        let mut lenb = [0u8; 4];
        self.stream.read_exact(&mut lenb)?;
        let len = u32::from_le_bytes(lenb) as usize;

        let mut usernameb = vec![0u8; len];
        self.stream.read_exact(&mut usernameb)?;
        self.opponent_username = String::from_utf8_lossy(&usernameb).to_string();

        Ok(())
    }

    pub fn check_for_updates(&mut self) -> Result<Update, ClientErr> {
        self.stream.set_nonblocking(true)?;

        let mut byte = [0u8; 1];
        let result = match self.stream.peek(&mut byte) {
            Ok(0) => Ok(Update::None),
            Ok(_) => {
                let mut update_typeb = [0u8; 1];
                self.stream.read_exact(&mut update_typeb)?;

                match update_typeb[0] {
                    x if x == Protocol::StartGame as u8 => Ok(Update::StartGame),
                    x if x == Protocol::YourTurn as u8 => Ok(Update::YourTurn),
                    x if x == Protocol::WaitTurn as u8 => Ok(Update::WaitTurn),
                    x if x == Protocol::GameOver as u8 => Ok(Update::GameOver),
                    _ => Ok(Update::None),
                }
            }
            Err(e) => Err(ClientErr::from(e)),
        };

        self.stream.set_nonblocking(false)?;

        result
    }
}
