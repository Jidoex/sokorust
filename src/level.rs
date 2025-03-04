use crate::tile::{Tile, floor::Floor, goal::Goal, hole::Hole, wall::Wall};
use std::{fmt::Debug, fs};

// Every level are 9 x 14 with border squares always being walls

pub struct Level {
    tiles: Vec<Vec<Tile>>,
    name: String,
}

impl Level {
    fn new(self, levelpath: &str) -> Self {
        let content =
            fs::read_to_string(levelpath).expect("Should have been able to read the file");
        let (tiles_string, name) = content
            .split_once("\n\n")
            .expect("Should have been able to split in two");

        let tiles: Vec<Vec<Tile>> = tiles_string
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '□' => Tile::Wall(Wall {}),
                        ' ' => Tile::Hole(Hole {}),
                        'O' => Tile::Goal(Goal {}),
                        '■' => Tile::Floor(Floor {}),
                        _ => Tile::Floor(Floor {}), // TODO for now every other characters are considered as floor
                    })
                    .collect()
            })
            .collect();

        Level {
            tiles,
            name: name.to_string(),
        }
    }
}