use crate::game_object::GameObject;

use super::moveable::Moveable;

#[derive(Clone, Copy)]
pub struct Egg {
    coord: [u8; 2],
}

impl GameObject for Egg {
    fn sprite(&self) -> char {
        'o'
    }

    fn coord(&self) -> [u8; 2] {
        self.coord
    }

    fn set_coord(&mut self, new_coord: [u8; 2]) {
        self.coord = new_coord;
    }
}

impl Moveable for Egg {}

impl Egg {
    fn new(coord: [u8; 2]) -> Self {
        Egg {coord}
    }
}