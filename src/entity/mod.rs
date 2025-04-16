pub mod egg;
pub mod maggot;
mod moveable;
pub mod player;
mod test;

use crate::game_object::GameObject;
use egg::Egg;
use maggot::Maggot;
use moveable::{Direction, Moveable};
use player::Player;

pub enum Entity<'a> {
    Player(&'a mut Player), //this mut is probably not what I want to do, but anyway
    Egg(&'a mut Egg),
    Maggot(&'a mut Maggot),
}

impl GameObject for Entity<'_> {
    fn sprite(&self) -> char {
        match self {
            Entity::Player(player) => player.sprite(),
            Entity::Egg(egg) => egg.sprite(),
            Entity::Maggot(maggot) => maggot.sprite(),
        }
    }

    fn coord(&self) -> [u8; 2] {
        match self {
            Entity::Player(player) => player.coord(),
            Entity::Egg(egg) => egg.coord(),
            Entity::Maggot(maggot) => maggot.coord(),
        }
    }

    fn set_coord(&mut self, new_coord: [u8; 2]) {
        match self {
            Entity::Player(player) => player.set_coord(new_coord),
            Entity::Egg(egg) => egg.set_coord(new_coord), //egg.set_coord(new_coord),
            Entity::Maggot(maggot) => maggot.set_coord(new_coord), //maggot.set_coord(new_coord),
        }
    }
}

impl Moveable for Entity<'_> {}

impl Entity<'_> {
    fn new_maggot<'a>(maggot: &'a mut Maggot) -> Entity<'a> {
        Entity::Maggot(maggot)
    }
}
