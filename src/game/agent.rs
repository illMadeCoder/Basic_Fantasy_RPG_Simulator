use crate::game::Game;
use crate::GameAction;

pub trait Agent {
    fn game_action(&self, target: &Game) -> GameAction;
}
