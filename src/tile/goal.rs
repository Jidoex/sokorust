use super::{Physics, TileObject};
use crate::game_object::GameObject;

#[derive(Clone, Copy)]
pub struct Goal {}

impl GameObject for Goal {
    const SPRITE: char = 'O';
}

impl TileObject for Goal {
    const PHYSICS: Physics = Physics::Walkable;
}
