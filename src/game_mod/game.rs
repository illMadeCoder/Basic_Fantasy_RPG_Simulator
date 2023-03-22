use crate::{
    character_mod::Character,
    dice_expr_mod::{Dice, DicePool},
    game_mod::Direction,
    monster::Monster,
};

use super::{
    direction::DIRECTIONS,
    game_object::{GameObjectId, IntoGameObject},
    GameAction, GameObject, Point,
};

type GameCharacterId = usize;

pub struct GameCharacter {
    id: GameCharacterId,
    position: Point,
    hp: i32,
    character: Character,
    turn: usize,
    name: String,
    ac: i32,
}

type GameMonsterId = usize;

pub struct GameMonster {
    id: GameMonsterId,
    hp: i32,
    position: Point,
    monster: Monster,
    turn: usize,
    name: String,
    ac: i32,
}

pub enum GameBody {
    Character(GameCharacterId),
    Monster(GameMonsterId),
}

pub struct Game {
    pub characters: Vec<GameCharacter>,
    pub monsters: Vec<GameMonster>,
    pub end: bool,
    pub turn: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            characters: Vec::new(),
            monsters: Vec::new(),
            end: false,
            turn: 0,
        }
    }

    pub fn get_monster_at(&self, p: &Point) -> Option<GameMonsterId> {
        self.monsters
            .iter()
            .find(|x| x.position == *p)
            .map(|x| x.id)
    }

    pub fn get_character_at(&self, p: &Point) -> Option<GameCharacterId> {
        self.characters
            .iter()
            .find(|x| x.position == *p)
            .map(|x| x.id)
    }

    pub fn get_at(&self, p: &Point) -> Option<GameBody> {
        if let Some(id) = self.get_character_at(p) {
            Some(GameBody::Character(id))
        } else if let Some(id) = self.get_monster_at(p) {
            Some(GameBody::Monster(id))
        } else {
            None
        }
    }

    pub fn take_turn(&mut self) {
        println!("turn {}", self.turn);

        if self.turn % 2 == 0 {
            let c = &self.characters[0];

            let stdin = std::io::stdin();
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            let trimmed = buf.trim().to_string();
            let mut split = trimmed.split(' ');
            let action = split.next().unwrap();
            if action == "move" {
                let dir_str = split.next().unwrap();
                let direction: Direction = dir_str.parse().unwrap();

                let new_position = c.position + Point::from(direction);
                self.characters[0].position = new_position;
            } else if action == "attack" {
                println!(
                    "{0} is attacking {1}",
                    self.characters[0].name, self.monsters[0].name
                );

                let roll = DicePool::new(1, Dice::D20).dice_roll_sum().sum;
                println!(
                    "{0} rolls to hit against AC {1}",
                    self.characters[0].name, roll
                );

                if roll > self.monsters[0].ac {
                    let damage = DicePool::new(1, Dice::D6).dice_roll_sum().sum;
                    println!("{0} rolls {1} for damage", self.characters[0].name, damage);
                    self.monsters[0].hp -= damage;
                    println!(
                        "{0} takes {1} damage and now has {2} hp",
                        self.monsters[0].name, damage, self.monsters[0].hp
                    );
                }
            }

            // let action = split.next().unwrap();
            // if action == "move" {
            //     let dir = split.next().unwrap();
            //     GameAction::Move {
            //         target: 0,
            //         direction: dir.parse().unwrap(),
            //     }
            // } else if action == "attack" {
            //     GameAction::MeleeAttack {
            //         source: 0,
            //         target: 1,
            //     }
            // } else {
            //     GameAction::None
            // }
        } else {
            println!(
                "{0} is attacking {1}",
                self.monsters[0].name, self.characters[0].name
            );

            let roll = DicePool::new(1, Dice::D20).dice_roll_sum().sum;
            println!(
                "{0} rolls to hit against AC {1}",
                self.monsters[0].name, roll
            );

            if roll > self.characters[0].ac {
                let damage = DicePool::new(1, Dice::D6).dice_roll_sum().sum;
                println!("{0} rolls {1} for damage", self.monsters[0].name, damage);
                self.characters[0].hp -= damage;
                println!(
                    "{0} takes {1} damage and now has {2} hp",
                    self.characters[0].name, damage, self.characters[0].hp
                );
            }
        }

        // if self.turn % 2 == 0 {
        //     self.apply(self.characters[0].next_action());
        // } else {
        //     self.apply(self.monsters[0].next_action());
        // }

        // if self.game_objects[0].get_hp() <= 0 {
        //     self.end = true;
        //     println!("{} won", self.game_objects[1].get_name())
        // } else if self.game_objects[1].get_hp() <= 0 {
        //     self.end = true;
        //     println!("{} won", self.game_objects[0].get_name())
        // }

        self.turn += 1;
    }

    fn surrounding(p: &Point) -> Vec<Point> {
        DIRECTIONS.iter().map(|dir| Point::from(dir) + *p).collect()
    }

    fn surrounding_game_objects(&self, p: &Point) -> Vec<GameObjectId> {
        todo!()
        // Game::surrounding(p)
        //     .iter()
        //     .filter_map(|s_p| self.get_at(s_p))
        //     .collect()
    }

    pub fn insert_character(&mut self, character: Character) {
        let id = self.characters.len();
        self.characters.push(GameCharacter {
            name: character.name.clone(),
            id,
            position: Point::new(2, 2),
            hp: character.hp,
            character,
            turn: 0,
            ac: 10,
        });
    }

    pub fn insert_monster(&mut self, monster: Monster) {
        let id = self.monsters.len();
        self.monsters.push(GameMonster {
            name: monster.name.clone(),
            id,
            position: Point::new(3, 2),
            hp: monster.hp,
            monster,
            turn: 1,
            ac: 10,
        });
    }

    pub fn apply(&mut self, game_action: GameAction) {
        todo!()
        // match game_action {
        //     GameAction::MeleeAttack { source, target } => {
        //         let source_position = self.game_objects[source].get_position();
        //         if !self
        //             .surrounding_game_objects(&source_position)
        //             .iter()
        //             .any(|x| *x == target)
        //         {
        //             println!("target is not in reach");
        //             return ();
        //         }

        //         println!(
        //             "{0} is attacking {1}",
        //             self.game_objects[source].get_name(),
        //             self.game_objects[target].get_name()
        //         );

        //         let roll = DicePool::new(1, Dice::D20).dice_roll_sum().sum;
        //         println!(
        //             "{0} rolls to hit against AC {1}",
        //             self.game_objects[source].get_name(),
        //             roll
        //         );

        //         if roll > self.game_objects[target].get_ac() {
        //             let damage = DicePool::new(1, Dice::D6).dice_roll_sum().sum;
        //             println!(
        //                 "{0} rolls {1} for damage",
        //                 self.game_objects[source].get_name(),
        //                 damage
        //             );
        //             self.game_objects[target].take_damage(damage);
        //             println!(
        //                 "{0} takes {1} damage and now has {2} hp",
        //                 self.game_objects[target].get_name(),
        //                 damage,
        //                 self.game_objects[target].get_hp()
        //             );
        //         }
        //     }

        //     GameAction::Move { target, direction } => {
        //         let prev_pos = self.game_objects[target].get_position();
        //         self.game_objects[target].displace(direction.into());
        //         let cur_pos = self.game_objects[target].get_position();
        //         println!("move from {:?} to {:?}", prev_pos, cur_pos);
        //     }

        //     GameAction::None => println!("do nothing"),
        // }
    }
}
