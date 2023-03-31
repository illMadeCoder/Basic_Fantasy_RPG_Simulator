use crate::game_mod::{Actor, Point};

use super::{Direction, Game};

#[derive(Debug)]
pub enum Action {
    Idle(usize),
    Move(usize, Direction),
    Attack(usize, usize),
}
