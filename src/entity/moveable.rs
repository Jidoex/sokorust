use crate::game_object::GameObject;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub trait Moveable: GameObject {
    fn hits_edge(&self, direction: Direction) -> bool {
        let coord = self.coord();
        match direction {
            Direction::North => coord[0] == 0,
            Direction::South => coord[0] >= 8, // height of a level is 9
            Direction::West => coord[1] == 0,
            Direction::East => coord[1] >= 13, // width of a level is 14
        }
    }

    fn next_tile(&self, direction: Direction) -> [u8; 2] {
        let curr_coord = self.coord();
        let mut next_coord = curr_coord;
        match direction {
            Direction::North => next_coord[0] -= 1,
            Direction::South => next_coord[0] += 1,
            Direction::East => next_coord[1] += 1,
            Direction::West => next_coord[1] -= 1,
        }
        next_coord
    }
}
