use super::{Action, Direction, Game, Point};

pub struct Actor {
    pub name: String,
    pub position: Point,
    pub behavior: Behavior,
    pub hp: i32,
}

pub enum Behavior {
    Idle,
    Wonder,
    Agressive,
    Player,
}

impl Actor {
    pub fn next_action(&self, actor_id: usize, game: &Game) -> Action {
        match self.behavior {
            Behavior::Idle => Action::Idle(actor_id),

            Behavior::Wonder => Action::Move(actor_id, Direction::rnd()),

            Behavior::Agressive => Action::Attack(
                actor_id,
                game.actors
                    .iter()
                    .enumerate()
                    .find(|&x| x.0 != actor_id)
                    .expect("can't find another to attack")
                    .0,
            ),

            Behavior::Player => {
                // TODO: Move to its own module
                let stdin = std::io::stdin();
                let mut buf = String::new();
                stdin.read_line(&mut buf).unwrap();
                let trimmed = buf.trim().to_string();
                let mut split = trimmed.split(' ');
                let action = split.next().unwrap();
                if action == "move" {
                    let dir_str = split.next().unwrap();
                    let direction: Direction = dir_str.parse().unwrap();
                    Action::Move(actor_id, direction)
                } else if action == "attack" {
                    let target_str = split.next().unwrap();
                    let target: usize = target_str.parse().unwrap();
                    Action::Attack(actor_id, target)
                } else {
                    Action::Idle(actor_id)
                }
            }
        }
    }
}
