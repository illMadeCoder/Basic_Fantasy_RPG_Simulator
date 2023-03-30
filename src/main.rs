#![allow(dead_code)]
mod character_mod;
mod dice_expr_mod;
mod game_mod;
mod input;
mod item;
mod monster;
mod view;

use character_mod::Character;
use game_mod::{Game, Actor, Point, Behavior};
use monster::Monster;

fn main() {
    let character = Character::gen();
    let monster = Monster::gen();

    let actor_character = Actor {
        position: Point { x: 0, y: 0 },
        hp: 10,
	behavior: Behavior::Wonder
    };

    let actor_monster = Actor {
        position: Point { x: 0, y: 0 },
        hp: 10,
	behavior: Behavior::Agressive
    };

    let mut game = Game::new(vec![actor_character, actor_monster]);
    
    loop {
	game.step();
    }
    
    // game.insert_character(c);
    // game.insert_monster(m);
    // view::draw(&game);
}
