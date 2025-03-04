use super::{Physics, TileObject};
use crate::game_object::GameObject;

#[derive(Clone, Copy)]
pub struct Hole {}

impl GameObject for Hole {
    const SPRITE: char = ' ';
}

impl TileObject for Hole {
    const PHYSICS: Physics = Physics::Walkable;
}
