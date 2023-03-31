#![allow(dead_code)]
mod character_mod;
mod dice_expr_mod;
mod game_mod;
mod input;
mod item;
mod monster;
mod view;

use character_mod::Character;
use core::time::Duration;
use game_mod::{Actor, Behavior, Game, Point};
use monster::Monster;
use std::{thread, time};

fn main() {
    let character = Character::gen();
    let monster = Monster::gen();

    let actor_character = Actor {
        name: character.name.clone(),
        position: Point { x: 2, y: 3 },
        hp: 10,
        behavior: Behavior::Player,
    };

    let actor_monster = Actor {
        name: monster.name.clone(),
        position: Point { x: 3, y: 5 },
        hp: 10,
        behavior: Behavior::Agressive,
    };

    let mut game = Game::new(vec![actor_character, actor_monster]);

    loop {
        view::draw(&game);
        game.step();
        thread::sleep(time::Duration::from_secs(1));
    }
}
