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

use std::collections::HashSet;

pub struct PrototypeAgent {
    actor: usize,
}

impl Agent for PrototypeAgent {
    fn decide_action(&self, game: &Game) -> GameAction {
        let character = game.get(self.actor);
        let potential_set = character.potential_action_set();
        if potential_set.contains(&PotentialAction::Attack) {
            GameAction::MeleeAttack {
                target: *game.get_by_name("Goblin".to_string()).first().unwrap(),
            }
        } else {
            GameAction::None
        }
    }
}

impl Actor for Character {
    fn potential_action_set(&self) -> HashSet<PotentialAction> {
        let mut set = HashSet::new();
        set.insert(PotentialAction::Attack);
        set
    }
}

impl Actor for Monster {
    fn potential_action_set(&self) -> HashSet<PotentialAction> {
        let set = HashSet::new();
        //set.insert(Potential::Attack);
        set
    }
}

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
}

fn main() {
    let mut character = Character::gen();
    let mut monster = Monster::gen();
    let mut game = Game::new();
    let character_id = game.insert(&mut character);
    let _monster_id = game.insert(&mut monster);

    let agent_character = PrototypeAgent {
        actor: character_id,
    };
    let action_character = agent_character.decide_action(&game);
    view::draw(&game);
    game.apply(action_character);
}
