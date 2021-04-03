mod maps;
mod player_input;

use maps::Map;

fn main() {
    let map = Map::load_file("src/maps/home.map".to_string());

    map.debug_render();
}
