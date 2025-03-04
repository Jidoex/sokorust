use super::{Physics, TileObject};
use crate::game_object::GameObject;

#[derive(Clone, Copy)]
pub struct Wall {}

impl GameObject for Wall {
    const SPRITE: char = '□';
}

impl TileObject for Wall {
    const PHYSICS: Physics = Physics::Static;
}
