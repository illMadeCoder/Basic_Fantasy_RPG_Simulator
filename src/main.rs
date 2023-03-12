#![allow(dead_code)]

mod character;
mod dice_expr;
mod game;
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
use std::rc::Rc;

// pub struct PrototypeAgent<'a> {
//     actor: Ref<&'a dyn GameObject>,
// }

// impl Agent for PrototypeAgent<'_> {
//     fn decide_action(&self, game: &Game) -> GameAction {
//         // let character = game.get(self.actor);
//         // let potential_set = character.potential_action_set();
//         // if potential_set.contains(&PotentialAction::Attack) {
//         //     GameAction::MeleeAttack {
//         //         target: *game.get_by_name("Goblin".to_string()).first().unwrap(),
//         //     }
//         // } else {
//         //     GameAction::None
//         // }
//         todo!()
//     }
// }

// impl Actor for Character {
//     fn potential_action_set(&self) -> HashSet<PotentialAction> {
//         let mut set = HashSet::new();
//         set.insert(PotentialAction::Attack);
//         set
//     }
// }

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
}

fn main() {
    let mut game = Game::new();
    let c: Rc<RefCell<dyn GameObject>> = Rc::new(RefCell::new(Character::gen()));
    let m: Rc<RefCell<dyn GameObject>> = Rc::new(RefCell::new(Monster::gen()));
    game.insert_at(Rc::clone(&c), Point::new(1, 1));
    game.insert_at(Rc::clone(&m), Point::new(2, 1));
    // game.insert_at(Rc::clone(&m), Point::new(2, 0));
    // game.apply(GameAction::MeleeAttack {
    //     source: Rc::clone(&c),
    //     target: Rc::clone(&m),
    // });

    // let agent_character = PrototypeAgent {
    //     actor: character.borrow(),
    // };
    //let action_character = agent_character.decide_action(&game);
    view::draw(&game);
    //game.apply(action_character);
}
