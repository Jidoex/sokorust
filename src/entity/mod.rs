pub mod cube;
pub mod player;

use crate::game_object::GameObject;

trait EntityObject: GameObject {
    fn controller(&self, direction: char);
}
