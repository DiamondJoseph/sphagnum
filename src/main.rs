use std::fs::File;

use crate::game::Game;

pub mod game;
pub mod tile;

fn main() {
    let data_dir = std::env::current_exe()
        .expect("Cannot find executable")
        .parent()
        .expect("Cannot find directory containing executable")
        .parent()
        .expect("Cannot find target dir")
        .parent()
        .expect("Cannot find root of project dir")
        .join("data");

    let tiles_file =
        File::open(data_dir.join("tiles.json")).expect("Unable to find {data_dir:?}/tiles.json");

    let tiles: Vec<tile::Tile> = serde_json::from_reader(tiles_file).expect("JSON badly formed!");
    let game = Game::random_colour(tiles);

    println!("{game:?}");
}
