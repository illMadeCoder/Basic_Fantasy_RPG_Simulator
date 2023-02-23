// TODO this actor relation feels wrong
use super::Actor;

pub trait GameObject: Actor {
    fn get_ac(&self) -> i32;
    fn get_name(&self) -> &str;
    fn take_damage(&mut self, damage: i32);
    fn get_hp(&self) -> i32;
}
