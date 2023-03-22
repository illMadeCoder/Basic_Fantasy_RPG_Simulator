#![allow(dead_code)]
mod character_mod;
mod dice_expr_mod;
mod game_mod;
mod grid;
mod item;
mod monster;
mod view;

use character_mod::Character;
use game_mod::Game;
use monster::Monster;

fn main() {
    let mut game = Game::new();
    let c = Character::gen();
    let m = Monster::gen();

    game.insert_character(c);
    game.insert_monster(m);
    while !game.end {
        view::draw(&game);
        game.take_turn();
    }
}
