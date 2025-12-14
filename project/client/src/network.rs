use std::{io::{Read, Write}, net::TcpStream};


#[repr(u8)]
enum Protocol {
    RequestRooms,
    StartRoom,
    JoinRoom,
    JoinSuccess,
}

type RoomData = Vec<([u8; 16], u8)>;

pub struct Network {
    stream: TcpStream,
}

impl Network {
    pub fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:1922")
            .expect("Could not connect to server!");

        Self {
            stream
        }
    }

    pub fn request_rooms(&mut self) -> Result<RoomData, std::io::Error> {
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

            println!("Room found with {} players", player_count);
        }

        Ok(room_data)
    }

    pub fn join_room(&mut self, room_id: &[u8; 16], username: String) -> Result<(), std::io::Error> {
        self.stream.write_all(&[Protocol::JoinRoom as u8])?;
        self.stream.write_all(room_id)?;
        self.stream.write_all(&(username.len() as u32).to_le_bytes())?;
        self.stream.write_all(&username.into_bytes())?;

        let mut responseb = [0u8; 1];
        self.stream.read_exact(&mut responseb)?;

        Ok(())
    }
}
