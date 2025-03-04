use crate::game_object::GameObject;

struct Player {}

impl GameObject for Player {
    const SPRITE: char = '@';
}
