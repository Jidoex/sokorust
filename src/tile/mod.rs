pub mod test;

use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    Hole,
    Exit,
    //TODO : Glass, Summonning, Traps, Smiting, Chests
}

impl Tile {
    fn sprite(self) -> char {
        match self {
            Tile::Floor => '■',
            Tile::Wall => '□',
            Tile::Hole => ' ',
            Tile::Exit => 'E',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            '■' => Tile::Floor,
            '□' => Tile::Wall,
            ' ' => Tile::Hole,
            'E' => Tile::Exit,
            _ => Tile::Floor,
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sprite = self.sprite();
        write!(f, "{sprite}")
    }
}
