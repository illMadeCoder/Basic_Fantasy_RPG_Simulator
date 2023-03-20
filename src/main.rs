#![allow(dead_code)]
mod character_mod;
mod dice_expr_mod;
mod direction;
mod game_mod;
mod grid;
mod item;
mod monster;
mod point;
mod view;

use character_mod::Character;
use game_mod::{Game, GameAction, GameObject};
use monster::Monster;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut game = Game::new();
    let c = Rc::new(RefCell::new(Character::gen()));
    let m = Rc::new(RefCell::new(Monster::gen()));
    game.insert(c);
    game.insert(m);
    while !game.over {
        view::draw(&game);
        game.take_turn();
    }
}
