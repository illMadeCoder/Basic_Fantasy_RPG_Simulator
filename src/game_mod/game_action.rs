use super::{game_object::GameObjectId, Direction};

pub enum GameAction {
    MeleeAttack {
        source: GameObjectId,
        target: GameObjectId,
    },
    Move {
        target: GameObjectId,
        direction: Direction,
    },
    None,
}
