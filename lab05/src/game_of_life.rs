use std::{fs, io, thread::sleep, time};

const MAX_EPISODES: u32 = 1000;
const HEIGHT: usize = 16;
const WIDTH:  usize = 16;

type Map = [[bool; WIDTH]; HEIGHT];

fn init_map(filepath: &str) -> Result<Map, io::Error> {
    let mut map: Map = [[false; WIDTH]; HEIGHT];

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

fn print_map(map: &Map, episode: &u32) {
    println!("Episode {}", episode);
    for line in map {
        for value in line {
            print!("{}", if !(*value) {' '} else { '#' });
        }
        println!();
    }
}

fn get_neighbours(map: &Map, i: usize, j: usize) -> u8 {
    let mut neighbours = 0;

    let offsets = vec![
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for (x, y) in offsets {
        let posx = i as i8 + x;
        let posy = j as i8 + y;

        if posx >= 0 && posy >= 0 && posx < WIDTH as i8 && posy < HEIGHT as i8
            && map[posx as usize][posy as usize] {
                neighbours += 1;
        }
    }

    neighbours
}


fn run(map: &mut Map) -> Result<(), io::Error> {
    let mut next_map = [[false; WIDTH]; HEIGHT];

    for episode in 0..MAX_EPISODES {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let n = get_neighbours(map, i, j);
                let is_alive = map[i][j];
            
                if is_alive {
                    next_map[i][j] = n == 2 || n == 3;
                } else {
                    next_map[i][j] = n == 3;
                }
            }
        }

        *map = next_map;

        print_map(map, &episode);
        sleep(time::Duration::from_millis(100));

        if let Err(e) = clearscreen::clear() {
            return Err(io::Error::other(e));
        }
    }

    Ok(())
}

pub fn start() {
    match init_map("game_start.txt") {
        Ok(mut map) => {
            if let Err(e) = run(&mut map) {
                println!("Eroare {}", e);
            }
        }
        Err(e)  => println!("Eroare {}", e)
    }
}