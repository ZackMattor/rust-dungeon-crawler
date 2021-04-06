mod maps;
mod player_input;

use maps::{Map, Point, TileType};
use player_input::{Command, Controls};
use std::io::{stdin, stdout, Write};
use std::{thread, time};
use termion::raw::IntoRawMode;

const loop_sleep_interval: time::Duration = time::Duration::from_millis(25);

fn render(map: &Map, player_location: &Point) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    print!("{}", termion::clear::All);

    for (y, row) in map.data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            print!("{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1));

            if player_location
                == &(Point {
                    x: x as i32,
                    y: y as i32,
                })
            {
                print!("☺");
            } else {
                match col {
                    TileType::Wall => print!("#"),
                    TileType::Floor => print!(" "),
                    TileType::Unknown => print!("?"),
                }
            }
        }

        print!("\n");
    }
}

fn main() {
    let map = Map::load_file("src/maps/home.map".to_string());
    let mut player_location = map.player_start.clone();

    let rx = Controls::start();

    loop {
        render(&map, &player_location);
        let received = rx.try_recv().unwrap_or(Command::Unknown);

        let movement_vector = match received {
            Command::MoveUp => Point { x: 0, y: -1 },
            Command::MoveLeft => Point { x: -1, y: 0 },
            Command::MoveDown => Point { x: 0, y: 1 },
            Command::MoveRight => Point { x: 1, y: 0 },
            Command::Unknown => Point { x: 0, y: 0 },
            Command::Exit => break,
        };

        let tmp_location = player_location.clone() + movement_vector.clone();

        if map.data[tmp_location.y as usize][tmp_location.x as usize] == TileType::Floor {
            player_location = player_location + movement_vector;
        }

        thread::sleep(loop_sleep_interval);
    }
}
