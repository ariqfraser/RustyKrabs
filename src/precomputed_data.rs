use std::{cmp::min, collections::HashMap};

use super::directions::DIRECTIONS;

pub struct PrecomputedData {
    pub moves_to_edge_hash: HashMap<(i32, &'static str), i32>,
    pub moves_to_edge: [MovesToEdge; 64],
}

impl Default for PrecomputedData {
    fn default() -> Self {
        PrecomputedData::calculate()
    }
}

impl PrecomputedData {
    pub fn calculate() -> Self {
        let mut data = PrecomputedData {
            moves_to_edge_hash: HashMap::new(),
            moves_to_edge: [MovesToEdge::default(); 64],
        };

        // Moves to edge
        for file in 0..8 {
            for rank in 0..8 {
                let position = rank * 8 + file;
                let moves_to_edge = MovesToEdge::new(file, rank);
                data.moves_to_edge[position as usize] = moves_to_edge;

                for dir in DIRECTIONS {
                    data.moves_to_edge_hash
                        .insert((position, dir), moves_to_edge.get(dir));
                }
            }
        }

        data
    }

    pub fn get_edge_distance(&self, pos: i32, direction: &str) -> i32 {
        self.moves_to_edge_hash
            .get(&(pos, direction))
            .unwrap()
            .to_owned()
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct MovesToEdge {
    pub north: i32,
    pub north_west: i32,
    pub north_east: i32,
    pub south: i32,
    pub south_east: i32,
    pub south_west: i32,
    pub east: i32,
    pub west: i32,
}

impl MovesToEdge {
    fn new(file: i32, rank: i32) -> Self {
        let i_file = file;
        let i_rank = rank;

        let south = 7 - i_rank;
        let north = i_rank;
        let east = 7 - i_file;
        let west = i_file;

        MovesToEdge {
            north,
            south,
            east,
            west,
            north_east: min(north, east),
            north_west: min(north, west),
            south_west: min(south, west),
            south_east: min(south, east),
        }
    }

    pub fn get(&self, key: &str) -> i32 {
        let val = match key.to_lowercase().as_str() {
            "north" => Some(self.north),
            "north_west" => Some(self.north_west),
            "north_east" => Some(self.north_east),
            "south" => Some(self.south),
            "south_east" => Some(self.south_east),
            "south_west" => Some(self.south_west),
            "east" => Some(self.east),
            "west" => Some(self.west),
            _ => Some(100),
        };

        if let Some(value) = val {
            value
        } else {
            0
        }
    }
}
