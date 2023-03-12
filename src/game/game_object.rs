use crate::point::Point;

pub trait GameObject {
    fn get_ac(&self) -> i32;
    fn get_name(&self) -> &str;
    fn take_damage(&mut self, damage: i32);
    fn get_hp(&self) -> i32;
    fn get_c(&self) -> char;
    fn get_position(&self) -> Point;
    fn set_position(&mut self, position: Point);
    fn displace(&mut self, vector: Point) {
        self.set_position(self.get_position() + vector);
    }
}
