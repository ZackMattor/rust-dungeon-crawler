mod maps;
mod player_input;

use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::{thread, time};
use maps::{Map, Point, TileType};
use player_input::{Command, Controls};

const loop_sleep_interval: time::Duration = time::Duration::from_millis(10);

fn render(map: &Map, player_location: &Point) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    for (y, row) in map.data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if map.player_start
                == (Point {
                    x: x as i32,
                    y: y as i32,
                })
            {
                print!(".");
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
    let player_location = map.player_start.clone();

    let rx = Controls::start();

    loop {
        render(&map, &player_location);
        let received = rx.try_recv().unwrap_or(Command::Unknown);

        match received {
            Command::Exit => break,
            Command::MoveUp => println!("Up"),
            Command::MoveLeft => println!("Left"),
            Command::MoveDown => println!("Down"),
            Command::MoveRight => println!("Right"),
            Command::Unknown => {},
        }

        thread::sleep(loop_sleep_interval);
    }
}
