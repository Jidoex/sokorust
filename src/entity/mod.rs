use std::fmt::Debug;

#[derive(Clone, Copy)]
enum Entity {
    Player,
    Egg,
    Snake,
    // TODO: the other enemies, NPCs and
}

impl Entity {
    pub fn get_sprite(self) -> char {
        match self {
            Entity::Player => '@',
            Entity::Egg => 'o',
            Entity::Snake => '-',
        }
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sprite = self.get_sprite();
        write!(f, "{sprite}")
    }
}
