use super::{
    direction::DIRECTIONS,
    game_object::{GameObjectId, IntoGameObject},
    GameAction, GameObject, Point,
};

type EntityId = usize;

pub struct Game {
    pub game_objects: Vec<GameObject>,
    pub turn: i32,
    pub end: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            game_objects: Vec::new(),
            turn: 0,
            end: false,
        }
    }

    pub fn get_at(&self, p: &Point) -> Option<GameObjectId> {
        self.game_objects
            .iter()
            .find(|x| x.get_position() == *p)
            .map(|x| x.get_id())
    }

    pub fn take_turn(&mut self) {
        println!("turn {}", self.turn);

        let stdin = std::io::stdin();
        if self.turn % 2 == 0 {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            let trimmed = buf.trim().to_string();
            let mut split = trimmed.split(' ');
            let action = split.next().unwrap();
            if action == "move" {
                let dir = split.next().unwrap();
                self.apply(GameAction::Move {
                    target: 0,
                    direction: dir.parse().unwrap(),
                });
            } else if action == "attack" {
                self.apply(GameAction::MeleeAttack {
                    source: 0,
                    target: 1,
                });
            } else {
                self.apply(GameAction::None);
            }
        } else {
            self.apply(GameAction::MeleeAttack {
                source: 1,
                target: 0,
            });
        }

        if self.game_objects[0].get_hp() <= 0 {
            self.end = true;
            println!("{} won", self.game_objects[1].get_name())
        } else if self.game_objects[1].get_hp() <= 0 {
            self.end = true;
            println!("{} won", self.game_objects[0].get_name())
        }

        self.turn += 1;
    }

    fn surrounding(p: &Point) -> Vec<Point> {
        DIRECTIONS.iter().map(|dir| Point::from(dir) + *p).collect()
    }

    fn surrounding_game_objects(&self, p: &Point) -> Vec<GameObjectId> {
        Game::surrounding(p)
            .iter()
            .filter_map(|s_p| self.get_at(s_p))
            .collect()
    }

    pub fn insert<T: IntoGameObject>(&mut self, game_object: T) {
        let game_object = game_object.into_game_object(self.game_objects.len());
        self.game_objects.push(game_object);
    }

    pub fn apply(&mut self, game_action: GameAction) {
        match game_action {
            GameAction::MeleeAttack { source, target } => {
                let source_position = self.game_objects[source].get_position();
                if !self
                    .surrounding_game_objects(&source_position)
                    .iter()
                    .any(|x| *x == target)
                {
                    println!("target is not in reach");
                    return ();
                }

                println!(
                    "{0} is attacking {1}",
                    self.game_objects[source].get_name(),
                    self.game_objects[target].get_name()
                );

                let roll = 15;
                println!(
                    "{0} rolls to hit against AC {1}",
                    self.game_objects[source].get_name(),
                    roll
                );

                if roll > self.game_objects[target].get_ac() {
                    let damage = 3;
                    println!(
                        "{0} rolls {1} for damage",
                        self.game_objects[source].get_name(),
                        damage
                    );
                    self.game_objects[target].take_damage(damage);
                    println!(
                        "{0} takes {1} damage and now has {2} hp",
                        self.game_objects[target].get_name(),
                        damage,
                        self.game_objects[target].get_hp()
                    );
                }
            }

            GameAction::Move { target, direction } => {
                let prev_pos = self.game_objects[target].get_position();
                self.game_objects[target].displace(direction.into());
                let cur_pos = self.game_objects[target].get_position();
                println!("move from {:?} to {:?}", prev_pos, cur_pos);
            }

            GameAction::None => println!("do nothing"),
        }
    }
}
