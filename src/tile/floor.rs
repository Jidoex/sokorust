use super::{Physics, TileObject};
use crate::game_object::GameObject;

#[derive(Clone, Copy)]
pub struct Floor {}

impl GameObject for Floor {
    const SPRITE: char = '■';
}

impl TileObject for Floor {
    const PHYSICS: Physics = Physics::Walkable;
}
