use std::{collections::HashSet, hash::Hash};

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct Planet {
    name: String,
    influence: u8,
    resources: u8,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub enum Anomaly {
    AsteroidField,
    EntropicScar,
    GravityRift,
    Nebula,
    Supernova,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum TileBack {
    Blue,
    Green,
    Red,
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub enum Wormhole {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tile {
    pub id: u8,
    pub back: TileBack,
    #[serde(default)]
    anomalies: HashSet<Anomaly>,
    #[serde(default)]
    planets: HashSet<Planet>,
    #[serde(default)]
    wormholes: HashSet<Wormhole>,
}

impl Tile {
    pub fn blank() -> Self {
        Self {
            id: 0,
            back: TileBack::Blue,
            anomalies: HashSet::new(),
            planets: HashSet::new(),
            wormholes: HashSet::new(),
        }
    }
}

pub fn get_mecatol() -> Tile {
    Tile {
            id: 18,
            back: TileBack::Blue,
            anomalies: HashSet::new(),
            wormholes: HashSet::new(),
            planets: HashSet::from(
                [Planet{name: "Mecatol Rex".to_string(), influence: 6, resources: 1}]
            )
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_planet() {
        let serialized = r#"{"name": "Jord", "influence": 2, "resources": 4}"#;
        let jord: Planet = serde_json::from_str(serialized).unwrap();
        assert_eq!(jord.influence, 2);
        assert_eq!(jord.resources, 4);
        assert_eq!(jord.name, "Jord");
    }

    #[test]
    fn deserialize_system() {
        let serialized = r#"{
        "back": "Green",
        "id": 1,
        "planets": [
            {
                "name": "Jord",
                "resources": 4,
                "influence": 2
            }
        ]
    }"#;
        let sol: Tile = serde_json::from_str(serialized).unwrap();
        assert!(sol.anomalies.is_empty());
        assert!(sol.wormholes.is_empty());
        assert_eq!(sol.back, TileBack::Green);

        assert_eq!(sol.planets.len(), 1);
        let jord = sol
            .planets
            .iter()
            .next()
            .expect("Unable to deserialize Jord from Sol system");
        assert_eq!(jord.influence, 2);
        assert_eq!(jord.resources, 4);
        assert_eq!(jord.name, "Jord");
    }
}
