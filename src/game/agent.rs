use crate::game::Game;
use crate::GameAction;

pub trait Agent {
    fn decide_action(&self, target: &Game) -> GameAction;
}
