use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{grid::Grid, point::Point};

use super::{GameAction, GameObject};

pub struct Game {
    pub grid: Grid<Rc<RefCell<dyn GameObject>>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            grid: Grid::new(4, 4),
        }
    }

    pub fn insert(&mut self, game_object: &Rc<RefCell<dyn GameObject>>) {
        let rc = Rc::clone(game_object);
        let position = rc.borrow().get_position();
        self.grid.insert(rc, position);
    }

    pub fn apply(&mut self, game_action: GameAction) {
        match game_action {
            GameAction::MeleeAttack { source, target } => {
                println!(
                    "{0} is attacking {1}",
                    source.borrow().get_name(),
                    target.borrow().get_name()
                );
                let roll = 15;
                println!(
                    "{0} rolls to hit against AC {1}",
                    source.borrow().get_name(),
                    roll
                );
                if roll > target.borrow().get_ac() {
                    let damage = 3;
                    println!(
                        "{0} rolls {1} for damage",
                        source.borrow().get_name(),
                        damage
                    );
                    (*target).borrow_mut().take_damage(damage);
                    println!(
                        "{0} takes {1} damage and now has {2} hp",
                        target.borrow().get_name(),
                        damage,
                        target.borrow().get_hp()
                    );
                }
            }
            GameAction::Move { target, vector } => {
                println!("move");
                let prev_pos = target.borrow().get_position();
                self.grid.remove(prev_pos);
                (*target).borrow_mut().displace(vector);
                let cur_pos = target.borrow().get_position();
                self.grid.insert(target, cur_pos);
            }
            GameAction::None => println!("do nothing"),
        }
    }
}
