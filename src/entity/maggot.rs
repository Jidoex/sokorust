use crate::game_object::GameObject;

use super::moveable::{Direction, Moveable};

/*
Ennemy that goes either North or South
if it hits an obstacle, such as a wall, a hole, the border of the level, etc
takes one turn to flip direction then starts moving again
*/
#[derive(Clone, Copy)]
pub struct Maggot {
    coord: [u8; 2],
    curr_direction: Direction,
}

impl GameObject for Maggot {
    fn sprite(&self) -> char {
        '|'
    }

    fn coord(&self) -> [u8; 2] {
        self.coord
    }

    fn set_coord(&mut self, new_coord: [u8; 2]) {
        self.coord = new_coord;
    }
}

impl Moveable for Maggot {}

impl Maggot {
    pub fn new(coord: [u8; 2], start_direction: Direction) -> Maggot {
        Maggot {
            coord,
            curr_direction: start_direction,
        }
    }

    pub fn flip(&mut self) {
        if self.curr_direction == Direction::North {
            self.curr_direction = Direction::South
        } else {
            self.curr_direction = Direction::North
        }
    }
}
