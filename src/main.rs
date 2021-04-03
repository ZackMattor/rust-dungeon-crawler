mod maps;
use maps::{Map};

fn main() {
    let map = Map::load("Home".to_string(), "src/maps/home.map".to_string());

    map.debug_render();
}
