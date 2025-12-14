use crate::room::Room;
use uuid::Uuid;
use std::collections::HashMap;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

#[derive(thiserror::Error, Debug)]
enum ServerErr {
    #[error("User disconnected")]
    UserDisconnected,

    #[error("Unknown Command")]
    UnknownCommand,

    #[error("Io error")]
    IO(#[from] std::io::Error),
}

#[repr(u8)]
enum Protocol {
    RequestRooms,
    StartRoom,
    JoinRoom,
    JoinSuccess,
}

struct User {
    // stream: TcpStream,
    username: String,
}

struct ServerState {
    users: HashMap<Uuid, User>,
    rooms: HashMap<Uuid, Room>,
}

impl ServerState {
    fn add_user(&mut self, _stream: TcpStream) -> Uuid {
        let id = Uuid::new_v4();
        
        self.users.insert(id, User {
            // stream,
            username: "guest".to_string(),
        });
        
        id
    }

    // pub fn get_user(&self, id: &Uuid) -> Option<&User> {
    //     self.users.get(id)
    // } 

    pub fn get_user_mut(&mut self, id: &Uuid) -> Option<&mut User> {
        self.users.get_mut(id)
    }
    
    pub fn remove_user(&mut self, id: &Uuid) -> Option<User> {
        self.users.remove(id)
    }
}

pub struct Controller {
    listener: TcpListener,
    state: Arc<Mutex<ServerState>>
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
        Self::add_room(&self.state);
        Self::add_room(&self.state);

        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    let state_clone = Arc::clone(&self.state);

                    let uid = match (state_clone.lock(), stream.try_clone()) {
                        (Ok(mut state_guard), Ok(clone)) => Some(state_guard.add_user(clone)),
                        _ => None
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

    fn handle_command(cmd: u8, stream: &mut TcpStream, uid: &Uuid, state: &Arc<Mutex<ServerState>>) -> Result<(), ServerErr> {
        // let mut buffer = [0; 1024];

        if cmd == Protocol::StartRoom as u8 {
            // let bytes_read = match stream.read(&mut buffer) {
            //     Ok(0) | Err(_) => return Err(ServerErr::UserDisconnected),
            //     Ok(n) => n,
            // };
            //
            // let username = String::from_utf8_lossy(&buffer[0..bytes_read]).into_owned();
            // println!("Start new room with user [{}]", username);
            // Self::set_name(uid, username, state);
            // Self::add_room(state);

        } else if cmd == Protocol::JoinRoom as u8 {
            Self::handle_join_command(stream, uid, state)?;

        } else if cmd == Protocol::RequestRooms as u8 {
            Self::handle_request_rooms_command(stream, state)?;
        } else {
            return Err(ServerErr::UnknownCommand);
        }

        Ok(())
    }

    fn handle_join_command(stream: &mut TcpStream, uid: &Uuid, state: &Arc<Mutex<ServerState>>) -> Result<(), ServerErr> {
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

        Ok(())
    }

    fn handle_request_rooms_command(stream: &mut TcpStream, state: &Arc<Mutex<ServerState>>) -> Result<(), ServerErr> {
        if let Ok(state_guard) = state.lock() {
            let mut buffer: Vec<u8> = Vec::new();
            let room_countb: [u8; 4] = (state_guard.rooms.len() as u32).to_le_bytes();
            buffer.write_all(&room_countb)?;

            for (room_id, room) in state_guard.rooms.iter() {
                let room_idb: [u8; 16] = room_id.to_bytes_le();
                buffer.write_all(&room_idb)?;

                let player_countb: [u8; 1] = [room.get_player_count()];
                buffer.write_all(&player_countb)?;
            }

            stream.write_all(&buffer)?;
        }

        Ok(())
    }

    fn set_name(uid: &Uuid, username: &str, state: &Arc<Mutex<ServerState>>) {
        if let Ok(mut state_guard) = state.lock() &&
            let Some(user) = state_guard.get_user_mut(uid) {
            user.username = username.to_string();
        }
    }

    fn add_room(state: &Arc<Mutex<ServerState>>) {
        if let Ok(mut state_guard) = state.lock() {
            state_guard.rooms.insert(Uuid::new_v4(), Room::new());
        }
    }

    fn add_user_to_room(user_id: &Uuid, room_id: &Uuid, state: &Arc<Mutex<ServerState>>) {
        if let Ok(mut state_guard) = state.lock()
            && let Some(room) = state_guard.rooms.get_mut(room_id)
        {
            room.add_player(user_id);
        }
    }
}
