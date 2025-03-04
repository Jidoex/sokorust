pub mod floor;
pub mod goal;
pub mod hole;
pub mod wall;

use crate::game_object::GameObject;
use floor::Floor;
use goal::Goal;
use hole::Hole;
use wall::Wall;

pub enum Physics {
    Static,
    Walkable,
}

pub trait TileObject: GameObject {
    const PHYSICS: Physics;
}

#[derive(Clone, Copy)]
pub enum Tile {
    Floor(Floor),
    Goal(Goal),
    Hole(Hole),
    Wall(Wall),
    Default,
}
