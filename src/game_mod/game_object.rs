use crate::point::Point;
use crate::Character;
use crate::Monster;

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

impl GameObject for Character {
    fn get_ac(&self) -> i32 {
        10
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_c(&self) -> char {
        'c'
    }

    fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

impl GameObject for Monster {
    fn get_ac(&self) -> i32 {
        10
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_c(&self) -> char {
        'm'
    }

    fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    fn get_position(&self) -> Point {
        self.position
    }
}
