use std::fs;

pub enum TileType {
    Wall,
    Floor,
    Unknown,
}

#[derive(PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Map {
    player_start: Point,
    data: Vec<Vec<TileType>>,
}

impl Map {
    pub fn load_file(path: String) -> Self {
        let data = fs::read_to_string(path).expect("Failed to load map");

        Self::from(&data)
    }

    pub fn debug_render(&self) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if self.player_start == ( Point { x: x as i32, y: y as i32 } ) {
                    print!("S");
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
}

impl From<&String> for Map {
    fn from(str_data: &String) -> Self {
        let mut data : Vec<Vec<TileType>> = vec![vec![]];
        let mut player_start = Point {
            x: 0,
            y: 0,
        };

        for c in str_data.chars() {
            match c {
                '#' => {
                    data.last_mut().unwrap().push(TileType::Wall);
                },
                '1' => {
                    data.last_mut().unwrap().push(TileType::Floor);
                    player_start = Point {
                        x: data.last().unwrap().len() as i32,
                        y: data.len() as i32
                    };
                },
                ' ' => {
                    data.last_mut().unwrap().push(TileType::Floor);
                },
                '\n' => {
                    data.push(vec![]);
                },
                _ => {
                    data.last_mut().unwrap().push(TileType::Unknown);
                    println!("DEBUG: '{}' is an unknown char found in map data", c);
                },
            }
        }

        Self {
            data,
            player_start,
        }
    }
}
