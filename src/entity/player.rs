use super::moveable::Moveable;
use crate::game_object::GameObject;

#[derive(Clone, Copy)]
pub struct Player {
    coord: [u8; 2],
}

impl GameObject for Player {
    fn sprite(&self) -> char {
        '@'
    }

    fn coord(&self) -> [u8; 2] {
        self.coord
    }

    fn set_coord(&mut self, new_coord: [u8; 2]) {
        self.coord = new_coord;
    }
}

impl Moveable for Player {}

impl Player {
    fn new(coord: [u8; 2]) -> Self{
        Self{coord}
    }
}