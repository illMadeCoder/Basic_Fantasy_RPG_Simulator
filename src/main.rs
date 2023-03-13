#![allow(dead_code)]

mod character;
mod dice_expr;
mod direction;
mod game;
mod grid;
mod item;
mod monster;
mod point;
mod view;

use character::Character;
use game::{Actor, Agent, PotentialAction};
use game::{Game, GameAction, GameObject};
use monster::Monster;
use point::Point;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct PrototypeAgent {
    actor: Rc<RefCell<dyn GameObject>>,
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

impl Actor for Character {
    fn potential_action_set(&self) -> HashSet<PotentialAction> {
        let mut set = HashSet::new();
        set.insert(PotentialAction::Attack);
        set
    }
}

// impl Actor for Monster {
//     fn potential_action_set(&self) -> HashSet<PotentialAction> {
//         let set = HashSet::new();
//         //set.insert(Potential::Attack);
//         set
//     }
// }

impl GameObject for Monster {
    fn get_ac(&self) -> i32 {
        10
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_c(&self) -> char {
        'm'
    }

    fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

impl GameObject for Character {
    fn get_ac(&self) -> i32 {
        10
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_c(&self) -> char {
        'c'
    }

    fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

fn main() {
    let mut game = Game::new();
    let c: Rc<RefCell<dyn GameObject>> = Rc::new(RefCell::new(Character::gen()));
    let m: Rc<RefCell<dyn GameObject>> = Rc::new(RefCell::new(Monster::gen()));
    game.insert(&c);
    game.insert(&m);

    let stdin = std::io::stdin();
    loop {
        view::draw(&game);
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let trimmed = buf.trim().to_string();
        let mut split = trimmed.split(' ');
        let action = split.next().unwrap();
        if action == "move" {
            let dir = split.next().unwrap();
            game.apply(GameAction::Move {
                target: Rc::clone(&c),
                direction: dir.parse().unwrap(),
            });
        } else if action == "attack" {
            game.apply(GameAction::MeleeAttack {
                source: Rc::clone(&c),
                target: Rc::clone(&m),
            });
        } else {
            game.apply(GameAction::None);
        }
    }

    // let agent_character = PrototypeAgent {
    //     actor: character.borrow(),
    // };
    // let action_character = agent_character.game_action(&game);

    //game.apply(action_character);
}
