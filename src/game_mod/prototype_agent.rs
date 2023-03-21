use super::{agent::Agent, game_object::GameObjectId, Game, GameAction};

pub struct PrototypeAgent {
    pub actor: GameObjectId,
}

impl Agent for PrototypeAgent {
    fn game_action(&self, game: &Game) -> GameAction {
        // let character = game.get(self.actor);
        // let potential_set = character.potential_action_set();
        // if potential_set.contains(&PotentialAction::Attack) {
        //     GameAction::MeleeAttack {
        //         target: *game.get_by_name("Goblin".to_string()).first().unwrap(),
        //     }
        // } else {
        //     GameAction::None
        // }
        GameAction::None
    }
}
