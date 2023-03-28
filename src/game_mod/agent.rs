use super::{game_object::GameObjectId, Direction, GameAction};

pub struct PrototypeAgent {
    pub actor: GameObjectId,
}

pub struct PlayerAgent {
    pub actor: GameObjectId,
}

pub enum Agent {
    PrototypeAgent { actor: GameObjectId },
    PlayerAgent { actor: GameObjectId },
}

impl Agent {
    pub fn next_action(&self) -> GameAction {
        match self {
            Agent::PrototypeAgent { actor } => GameAction::MeleeAttack {
                source: super::GameAttackSource::Monster(0),
                target: super::GameAttackTarget::Character(0),
            },
            Agent::PlayerAgent { actor } => {
                let stdin = std::io::stdin();
                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                let trimmed = buf.trim().to_string();
                let mut split = trimmed.split(' ');
                let action = split.next().unwrap();
                if action == "move" {
                    let dir = split.next().unwrap();
                    GameAction::Move {
                        body: super::GameBody::Character(0),
                        direction: dir.parse().unwrap(),
                    }
                } else if action == "attack" {
                    GameAction::MeleeAttack {
                        source: super::GameAttackSource::Character(0),
                        target: super::GameAttackTarget::Monster(0),
                    }
                } else {
                    GameAction::None
                }
            }
        }
    }
}
