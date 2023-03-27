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

trait Position {
    fn get_position(&self) -> Point;
    fn set_position(&mut self, p: Point);
}

impl Position for GameCharacter {
    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, p: Point) {
        self.position = p;
    }
}

impl Position for GameMonster {
    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, p: Point) {
        self.position = p;
    }
}

pub enum GameBody {
    Character(GameCharacterId),
    Monster(GameMonsterId),
}

pub enum GameAttackTarget {
    Character(GameCharacterId),
    Monster(GameMonsterId),
}

pub enum GameAttackSource {
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

    pub fn step(&mut self) {
        println!("turn {}", self.turn);

        if self.turn % 2 == 0 {
            let c = &self.characters[0];

            self.apply(crate::input::game_action());
        } else {
            let m = &self.monsters[0];

            self.apply(GameAction::MeleeAttack {
                source: GameAttackSource::Monster(0),
                target: GameAttackTarget::Character(0),
            });
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
        match game_action {
            GameAction::None => println!("do nothing"),

            GameAction::MeleeAttack { source, target } => match source {
                GameAttackSource::Character(source_id) => match target {
                    GameAttackTarget::Monster(target_id) => {
                        let c = &mut self.monsters[source_id];
                        let m = &mut self.characters[target_id];
                        let roll = DicePool::new(1, Dice::D20).dice_roll_sum().sum;

                        println!("{0} is attacking {1}", m.name, c.name);

                        let roll = DicePool::new(1, Dice::D20).dice_roll_sum().sum;
                        println!("{0} rolls {1} to hit against AC {2}", m.name, roll, m.ac);

                        if roll >= c.ac {
                            let damage = DicePool::new(1, Dice::D6).dice_roll_sum().sum;
                            println!("{0} rolls {1} for damage", m.name, damage);
                            c.hp -= damage;
                            println!(
                                "{0} takes {1} damage and now has {2} hp",
                                c.name, damage, c.hp
                            );
                        }
                    }
                    GameAttackTarget::Character(_) => todo!(),
                },
                GameAttackSource::Monster(source_id) => match target {
                    GameAttackTarget::Character(target_id) => {
                        let m = &mut self.monsters[source_id];
                        let c = &mut self.characters[target_id];
                        let roll = DicePool::new(1, Dice::D20).dice_roll_sum().sum;

                        println!("{0} is attacking {1}", m.name, c.name);

                        let roll = DicePool::new(1, Dice::D20).dice_roll_sum().sum;
                        println!("{0} rolls {1} to hit against AC {2}", m.name, roll, m.ac);

                        if roll >= c.ac {
                            let damage = DicePool::new(1, Dice::D6).dice_roll_sum().sum;
                            println!("{0} rolls {1} for damage", m.name, damage);
                            c.hp -= damage;
                            println!(
                                "{0} takes {1} damage and now has {2} hp",
                                c.name, damage, c.hp
                            );
                        }
                    }
                    GameAttackTarget::Monster(_) => todo!(),
                },
            },

            GameAction::Move { target, direction } => match target {
                GameBody::Character(id) => {
                    let c = &mut self.characters[id];
                    let new_position = c.position + Point::from(direction);
                    println!("move from {:?} to {:?}", c.position, new_position);
                    c.position = new_position;
                }
                GameBody::Monster(id) => {
                    let m = &mut self.monsters[id];
                    let new_position = m.position + Point::from(direction);
                    println!("move from {:?} to {:?}", m.position, new_position);
                    m.position = new_position;
                }
            },
        }
    }
}
