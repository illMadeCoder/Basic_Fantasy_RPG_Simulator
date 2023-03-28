use super::game::{GameAttackSource, GameAttackTarget};
use super::game_object::GameObjectId;
use super::{Direction, GameBody};

pub enum GameAction {
    MeleeAttack {
        source: GameAttackSource,
        target: GameAttackTarget,
    },
    Move {
        body: GameBody,
        direction: Direction,
    },
    None,
}
