use hexx::{Hex, storage::HexagonalMap};
use rand::Rng;

use crate::tile::{Tile, TileBack, get_mecatol};

#[derive(Debug)]
pub struct Game {
    map: HexagonalMap<Tile>,
}

impl Game {
    fn blank() -> Self {
        return Self {
            map: HexagonalMap::new(Hex::ZERO, 3, |_| Tile::blank()),
        };
    }

    pub fn random_colour(tiles: Vec<Tile>) -> Self {
        return Self {
            map: HexagonalMap::new(Hex::ZERO, 3, |h| {
                random_coloured_tile(h, tiles.clone(), rand::rng())
            }),
        };
    }
}

fn is_corner(hex: Hex) -> bool {
    hex.distance_to(Hex::ZERO) == 3
    && (hex.x() == 0 || hex.x().abs() == 3)
    && (hex.y() == 0 || hex.y().abs() == 3)
}

fn random_coloured_tile<R: Rng>(hex: Hex, tiles: Vec<Tile>, rng: R) -> Tile {
    if hex.distance_to(Hex::ZERO) == 0 {
        return get_mecatol();
    }
    if is_corner(hex) {
        return pop_tile(tiles, rng, |t| t.back == TileBack::Green);
    }
    return pop_tile(tiles, rng, |t| t.back != TileBack::Green);
}

fn pop_tile<R: Rng>(mut tiles: Vec<Tile>, mut rng: R, mut conditional: impl FnMut(&Tile) -> bool) -> Tile {
    let mut index = rng.random_range(..tiles.len()); 
    while !conditional(tiles.get(index).expect("")) {
        index = rng.random_range(..tiles.len());
    }
    tiles.swap_remove(index)
}
