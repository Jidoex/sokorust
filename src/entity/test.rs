use crate::entity::moveable::Direction;
use crate::game_object::GameObject;
use super::maggot::Maggot;
use super::{Entity, maggot};

#[test]
fn set_coord_maggot() {
    let mut maggot = Maggot::new([0, 0], Direction::South);
    maggot.set_coord([0, 1]);
    assert_eq!(maggot.coord(), [0, 1]);
}

#[test]
fn new_entity_maggot() {
    let mut maggot:  Maggot = Maggot::new([0, 0], Direction::South);
    let entity_maggot = Entity::new_maggot(&mut maggot);

    assert_eq!(entity_maggot.coord(), [0, 0]);
}

#[test]
fn set_coord_entity_maggot() {
    let mut maggot = Maggot::new([0, 0], Direction::South);
    let mut entity_maggot = Entity::new_maggot(&mut maggot);
    entity_maggot.set_coord([0, 1]);
    assert_eq!(entity_maggot.coord(), [0, 1]);
}
