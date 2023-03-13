use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{
    direction::{Direction, DIRECTIONS},
    grid::Grid,
    point::Point,
};

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

    fn surrounding(p: &Point) -> Vec<Point> {
        DIRECTIONS.iter().map(|dir| Point::from(dir) + *p).collect()
    }

    fn surrounding_game_objects(&self, p: &Point) -> Vec<&Rc<RefCell<dyn GameObject>>> {
        Game::surrounding(p)
            .iter()
            .filter_map(|s_p| self.grid.get(s_p))
            .collect()
    }

    pub fn insert(&mut self, game_object: &Rc<RefCell<dyn GameObject>>) {
        let rc = Rc::clone(game_object);
        let position = rc.borrow().get_position();
        self.grid.insert(rc, &position);
    }

    pub fn apply(&mut self, game_action: GameAction) {
        match game_action {
            GameAction::MeleeAttack { source, target } => {
                let source_position = source.borrow().get_position();
                if !self
                    .surrounding_game_objects(&source_position)
                    .iter()
                    .any(|x| Rc::ptr_eq(x, &target))
                {
                    println!("target is not in reach");
                    return ();
                }

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

            GameAction::Move { target, direction } => {
                let prev_pos = target.borrow().get_position();
                self.grid.remove(&prev_pos);
                (*target).borrow_mut().displace(direction.into());
                let cur_pos = target.borrow().get_position();
                self.grid.insert(target, &cur_pos);
                println!("move from {:?} to {:?}", prev_pos, cur_pos);
            }

            GameAction::None => println!("do nothing"),
        }
    }
}
