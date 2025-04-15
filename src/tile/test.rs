use crate::tile::Tile;

#[test]
fn from_char_floor() {
    let floor = Tile::from_char('■');
    assert_eq!(floor, Tile::Floor)
}

#[test]
fn from_char_any() {
    let floor = Tile::from_char('è');
    assert_eq!(floor, Tile::Floor)
}
