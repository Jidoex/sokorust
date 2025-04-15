use crate::{level::Level, tile::Tile};

#[test]
fn from_text_all_floor() {
    let all_floor = Level::from_text("./example_data/all_floor.txt");
    assert_eq!(all_floor.id_number, 65535);
    assert_eq!(all_floor.starting_point, [0, 0]);
    let mut tiles_eq = true;
    for row in all_floor.tiles {
        for tile in row {
            tiles_eq = tiles_eq && (tile == Tile::Floor);
        }
    }
    assert!(tiles_eq);
}
