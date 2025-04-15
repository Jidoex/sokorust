pub mod test;

use crate::tile::{self, Tile};
use std::{fmt::Debug, fs, io, sync::RwLockWriteGuard};

// Every level are 9 x 14 with border squares always being walls

pub struct Level {
    id_number: u16,
    starting_point: [u8; 2],
    tiles: Vec<Vec<Tile>>,
    // entities: [[Entity; 14]; 9],
}

// TODO use Result ? and retropagation instead
impl Level {
    pub fn from_text(path: &str) -> Self {
        let binding = fs::read_to_string(path).expect("Could not open file");
        let content: Vec<&str> = binding.split("\n\n").collect();

        // extracting the id of the level
        let id: u16 = str::parse(content[0]).expect("Could not parse id into a u16.");

        // extracting the starting position of the player in that room
        let raw_coord: Vec<&str> = content[2].split(" ").collect();
        let mut starting_point: [u8; 2] = [0; 2];
        for i in 0..2 {
            starting_point[i] = str::parse(raw_coord[i]).expect("Could not parse as u8");
        }

        // extraction the tiles type
        let tiles: Vec<Vec<Tile>> = content[1]
            .split(" ")
            .map(|row: &str| row.chars().map(|c: char| Tile::from_char(c)).collect())
            .collect();

        Level {
            id_number: id,
            starting_point,
            tiles,
        }
    }
}

// TODO, get it to print properly
impl Debug for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Level")
            .field("id_number", &self.id_number)
            .field("tiles", &self.tiles)
            .field("starting_point", &self.starting_point)
            .finish()
    }
}
