use crate::game_object::GameObject;

struct Cube {}

impl GameObject for Cube {
    const SPRITE: char = 'H';
}
