#![allow(dead_code)]
use std::{thread, time};

use basic_fantasy_rpg_simulator::{
    character_mod::Character,
    game_mod::{Actor, Behavior, Game, Point},
    monster::Monster,
    view,
};

fn main() -> ! {
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
