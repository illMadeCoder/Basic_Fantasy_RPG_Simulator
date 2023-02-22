use crate::{Game, GameAction};

pub trait Agent {
    fn decide_action(&self, target: &Game) -> GameAction;
}
