pub trait GameObject {
    fn sprite(&self) -> char;
    fn coord(&self) -> [u8; 2];

    fn set_coord(&mut self, new_coord: [u8; 2]);
}
