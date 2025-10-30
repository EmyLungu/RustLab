use std::{fs, io};

type Map = [[bool; 10]; 10];

fn init_map(filepath: &str) -> Result<Map, io::Error> {
    let mut map: Map = [[false; 10]; 10];

    let content = fs::read_to_string(filepath)?;

    for (i, line) in content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '0' => map[i][j] = false,
                '1' => map[i][j] = true,
                _ => {}
            }
        }
    }

    Ok(map)
}

pub fn run() {
    match init_map("game_start.txt") {
        Ok(map) => {
            for line in map {
                println!("{:?}", line);
            }
        },
        Err(e)  => println!("Eroare {}", e)
    }
}