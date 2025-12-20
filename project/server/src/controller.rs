use crate::room::{Room, TurnResult};
use std::collections::HashMap;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[repr(u8)]
enum Protocol {
    RequestRooms,
    StartRoomBot,
    JoinRoom,
    JoinSuccess,
    RequestTiles,
    Turn,
    WaitTurn,
    YourTurn,
    GameOver,
}

#[derive(thiserror::Error, Debug)]
enum ServerErr {
    // #[error("User disconnected")]
    // UserDisconnected,
    #[error("Unknown Command")]
    UnknownCommand,

    #[error("Io error")]
    IO(#[from] std::io::Error),
}

struct User {
    pub stream: TcpStream,
    username: String,
    pub room: Option<Uuid>,
}

struct ServerState {
    users: HashMap<Uuid, User>,
    rooms: HashMap<Uuid, Room>,
}

impl ServerState {
    fn add_user(&mut self, stream: TcpStream) -> Uuid {
        let id = Uuid::new_v4();

        self.users.insert(
            id,
            User {
                stream,
                username: "guest".to_string(),
                room: None,
            },
        );

        id
    }

    pub fn remove_user(&mut self, id: &Uuid) -> Option<User> {
        self.users.remove(id)
    }

    pub fn get_user_room(&self, id: &Uuid) -> Option<Uuid> {
        if let Some(user) = self.users.get(id) {
            user.room
        } else {
            None
        }
    }
}

pub struct Controller {
    listener: TcpListener,
    state: Arc<Mutex<ServerState>>,
}

impl Controller {
    pub fn new() -> Self {
        let addr = String::from("127.0.0.1:1922");
        let listener = TcpListener::bind(&addr).expect("Failed to bind to address!");

        println!("Server listening on {}", addr);

        Self {
            listener,
            state: Arc::new(Mutex::new(ServerState {
                users: HashMap::new(),
                rooms: HashMap::new(),
            })),
        }
    }

    pub fn run(&mut self) {
        Self::add_room(2, &self.state);
        Self::add_room(2, &self.state);

        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    let state_clone = Arc::clone(&self.state);

                    let uid = match (state_clone.lock(), stream.try_clone()) {
                        (Ok(mut state_guard), Ok(clone)) => Some(state_guard.add_user(clone)),
                        _ => None,
                    };

                    if let Some(uid) = uid {
                        std::thread::spawn(move || {
                            Self::handle_user(stream, uid, state_clone);
                        });
                    }
                }
                Err(e) => eprintln!("Failed to accept connection {}", e),
            }
        }
    }

    fn handle_user(mut stream: TcpStream, uid: Uuid, state: Arc<Mutex<ServerState>>) {
        let peer_addr = stream
            .peer_addr()
            .map_or_else(|_| "unknown".to_string(), |addr| addr.to_string());
        println!("Handleing connection from: {}", peer_addr);

        loop {
            let mut byte = [0u8; 1];
            if stream.read_exact(&mut byte).is_err() {
                break;
            }

            if Self::handle_command(byte[0], &mut stream, &uid, &state).is_err() {
                break;
            }
        }

        if let Ok(mut state_guard) = state.lock() {
            state_guard.remove_user(&uid);
        }
    }

    fn handle_command(
        cmd: u8,
        stream: &mut TcpStream,
        uid: &Uuid,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        if cmd == Protocol::StartRoomBot as u8 {
            Self::handle_new_bot_game(stream, uid, state)?;
        } else if cmd == Protocol::JoinRoom as u8 {
            Self::handle_join(stream, uid, state)?;
        } else if cmd == Protocol::RequestRooms as u8 {
            Self::handle_request_rooms(stream, state)?;
        } else if cmd == Protocol::RequestTiles as u8 {
            Self::handle_request_tiles(stream, state)?;
        } else if cmd == Protocol::Turn as u8 {
            Self::handle_turn(stream, uid, state)?;
        } else {
            return Err(ServerErr::UnknownCommand);
        }

        Ok(())
    }

    fn handle_new_bot_game(
        stream: &mut TcpStream,
        uid: &Uuid,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        let mut lenb = [0u8; 4];
        stream.read_exact(&mut lenb)?;
        let len = u32::from_le_bytes(lenb) as usize;

        let mut usernameb = vec![0u8; len];
        stream.read_exact(&mut usernameb)?;

        let username = String::from_utf8_lossy(&usernameb);

        Self::set_name(uid, &username, state);

        if let Some(room_id) = Self::add_room(1, state) {
            Self::add_user_to_room(uid, &room_id, state);

            let successb: [u8; 1] = [Protocol::JoinSuccess as u8];
            stream.write_all(&successb)?;
            stream.write_all(&room_id.to_bytes_le())?;

            Self::check_start_room(&room_id, state)?;

            println!("User [{}] started new bot game in [{}]", username, room_id);

            stream.write_all(&[Protocol::YourTurn as u8])?;
        }
        Ok(())
    }

    fn handle_join(
        stream: &mut TcpStream,
        uid: &Uuid,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        let mut room_idb = [0u8; 16];
        stream.read_exact(&mut room_idb)?;
        let room_id = Uuid::from_bytes_le(room_idb);

        let mut lenb = [0u8; 4];
        stream.read_exact(&mut lenb)?;
        let len = u32::from_le_bytes(lenb) as usize;

        let mut usernameb = vec![0u8; len];
        stream.read_exact(&mut usernameb)?;

        let username = String::from_utf8_lossy(&usernameb);
        println!("User [{}] wants to join [{}]", username, room_id);

        Self::set_name(uid, &username, state);
        Self::add_user_to_room(uid, &room_id, state);

        let successb: [u8; 1] = [Protocol::JoinSuccess as u8];
        stream.write_all(&successb)?;

        Self::check_start_room(&room_id, state)?;

        Ok(())
    }

    fn check_start_room(room_id: &Uuid, state: &Arc<Mutex<ServerState>>) -> Result<(), ServerErr> {
        let player_ids = if let Ok(state_guard) = state.lock()
            && let Some(room) = state_guard.rooms.get(room_id)
            && room.get_player_count() == 2
        {
            Some((room.players[0], room.players[1]))
        } else {
            None
        };

        if let Ok(mut state_guard) = state.lock()
            && let Some((pid1, pid2)) = player_ids
        {
            if let Some(player1) = state_guard.users.get_mut(&pid1) {
                player1.stream.write_all(&[Protocol::YourTurn as u8])?;
            }
            if let Some(player2) = state_guard.users.get_mut(&pid2) {
                player2.stream.write_all(&[Protocol::WaitTurn as u8])?;
            }
        }

        if player_ids.is_some() {
            Self::add_room(2, state);
        }

        Ok(())
    }

    fn handle_request_rooms(
        stream: &mut TcpStream,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        if let Ok(state_guard) = state.lock() {
            let mut rooms_buf: Vec<u8> = Vec::new();

            let mut count: u32 = 0;
            for (room_id, room) in state_guard.rooms.iter() {
                if room.is_available() {
                    let room_idb: [u8; 16] = room_id.to_bytes_le();
                    rooms_buf.write_all(&room_idb)?;

                    let player_countb: [u8; 1] = [room.get_player_count()];
                    rooms_buf.write_all(&player_countb)?;

                    count += 1;
                }
            }

            let mut data: Vec<u8> = Vec::new();
            let room_countb: [u8; 4] = count.to_le_bytes();
            data.write_all(&room_countb)?;
            data.extend(rooms_buf);
            stream.write_all(&data)?;
        }

        Ok(())
    }

    fn handle_request_tiles(
        stream: &mut TcpStream,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        let mut room_idb = [0u8; 16];
        stream.read_exact(&mut room_idb)?;
        let room_id = Uuid::from_bytes_le(room_idb);

        if let Ok(state_guard) = state.lock()
            && let Some(room) = state_guard.rooms.get(&room_id)
        {
            let data = room.get_grid();

            stream.write_all(&data)?;
        }

        Ok(())
    }

    fn handle_turn(
        stream: &mut TcpStream,
        uid: &Uuid,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        let mut bytes = [0u8; 4];
        stream.read_exact(&mut bytes)?;
        let y = u32::from_le_bytes(bytes) as usize;

        stream.read_exact(&mut bytes)?;
        let x = u32::from_le_bytes(bytes) as usize;

        let room_id: Option<Uuid> = if let Ok(state_guard) = state.lock() {
            state_guard.get_user_room(uid)
        } else {
            None
        };

        if let Some(rid) = room_id {
            let player_count = if let Ok(state_guard) = state.lock()
                && let Some(room) = state_guard.rooms.get(&rid)
            {
                Some(room.max_players)
            } else {
                None
            };

            if let Some(player_count) = player_count {
                if player_count == 1 {
                    Self::handle_singe_turn(stream, uid, &y, &x, &rid, state)?;
                } else {
                    Self::handle_multi_turn(stream, uid, &y, &x, &rid, state)?;
                }
            }
        }
        // if let Ok(mut state_guard) = state.lock() {
        //     let room_id = match state_guard.find_user_room(uid) {
        //         Some(id) => id,
        //         None => {
        //             stream.write_all(&[Protocol::WaitTurn as u8])?;
        //             return Ok(());
        //         }
        //     };
        //
        //     if let Some(room) = state_guard.rooms.get_mut(&room_id) {
        //         if room.max_players == 1 {
        //         } else {
        //             match room.process_turn(uid, &y, &x) {
        //                 TurnResult::Good => {
        //                     stream.write_all(&[Protocol::WaitTurn as u8])?;
        //
        //                     if let Some(other_player_id) = room.get_other_player(uid)
        //                     && let Some(other_player) = state_guard.users.get_mut(&other_player_id)
        //                     {
        //                         other_player.stream.write_all(&[Protocol::YourTurn as u8])?;
        //                     }
        //                 }
        //                 TurnResult::Bad => {}
        //                 TurnResult::GameOver => {
        //                     stream.write_all(&[Protocol::GameOver as u8])?;
        //
        //                     if let Some(other_player_id) = room.get_other_player(uid)
        //                     && let Some(other_player) = state_guard.users.get_mut(&other_player_id)
        //                     {
        //                         other_player.stream.write_all(&[Protocol::GameOver as u8])?;
        //                     }
        //
        //                     state_guard.rooms.remove(&room_id);
        //                 }
        //             }
        //         }
        //     }
        // }

        Ok(())
    }

    fn handle_singe_turn(
        stream: &mut TcpStream,
        uid: &Uuid,
        y: &usize,
        x: &usize,
        room_id: &Uuid,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        if let Ok(mut state_guard) = state.lock()
            && let Some(room) = state_guard.rooms.get_mut(room_id)
        {
            match room.process_turn(uid, y, x) {
                TurnResult::Good => {
                    if room.move_mouse_random() == TurnResult::GameOver {
                        stream.write_all(&[Protocol::GameOver as u8])?;
                        state_guard.rooms.remove(room_id);
                    } else {
                        stream.write_all(&[Protocol::YourTurn as u8])?;
                    }
                }
                TurnResult::Bad => {}
                TurnResult::GameOver => {
                    stream.write_all(&[Protocol::GameOver as u8])?;
                    state_guard.rooms.remove(room_id);
                }
            }
        }

        Ok(())
    }

    fn handle_multi_turn(
        stream: &mut TcpStream,
        uid: &Uuid,
        y: &usize,
        x: &usize,
        room_id: &Uuid,
        state: &Arc<Mutex<ServerState>>,
    ) -> Result<(), ServerErr> {
        if let Ok(mut state_guard) = state.lock()
            && let Some(room) = state_guard.rooms.get_mut(room_id)
        {
            match room.process_turn(uid, y, x) {
                TurnResult::Good => {
                    stream.write_all(&[Protocol::WaitTurn as u8])?;

                    if let Some(other_player_id) = room.get_other_player(uid)
                        && let Some(other_player) = state_guard.users.get_mut(&other_player_id)
                    {
                        other_player.stream.write_all(&[Protocol::YourTurn as u8])?;
                    }
                }
                TurnResult::Bad => {}
                TurnResult::GameOver => {
                    stream.write_all(&[Protocol::GameOver as u8])?;

                    if let Some(other_player_id) = room.get_other_player(uid)
                        && let Some(other_player) = state_guard.users.get_mut(&other_player_id)
                    {
                        other_player.stream.write_all(&[Protocol::GameOver as u8])?;
                    }

                    state_guard.rooms.remove(room_id);
                }
            }
        }

        Ok(())
    }

    fn set_name(uid: &Uuid, username: &str, state: &Arc<Mutex<ServerState>>) {
        if let Ok(mut state_guard) = state.lock()
            && let Some(user) = state_guard.users.get_mut(uid)
        {
            user.username = username.to_string();
        }
    }

    fn add_room(num_players: u8, state: &Arc<Mutex<ServerState>>) -> Option<Uuid> {
        if let Ok(mut state_guard) = state.lock() {
            let id = Uuid::new_v4();
            state_guard.rooms.insert(id, Room::new(num_players));

            Some(id)
        } else {
            None
        }
    }

    fn add_user_to_room(user_id: &Uuid, room_id: &Uuid, state: &Arc<Mutex<ServerState>>) {
        if let Ok(mut state_guard) = state.lock()
            && let Some(room) = state_guard.rooms.get_mut(room_id)
        {
            room.add_player(user_id);
            if let Some(user) = state_guard.users.get_mut(user_id) {
                user.room = Some(*room_id);
            }
        }
    }
}
