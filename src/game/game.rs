use std::{cell::RefCell, rc::Rc};

use crate::point::Point;

use super::{GameAction, GameObject};

pub struct Grid<T> {
    pub vec: Vec<Option<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let vec: Vec<Option<T>> = (0..width * height).map(|_| None).collect();
        Grid { vec, width, height }
    }

    fn point_to_index(&self, Point { x, y }: Point) -> usize {
        x + y * self.width
    }

    pub fn insert_at(&mut self, element: T, p: Point) {
        let i = self.point_to_index(p);
        self.vec.insert(i, Some(element))
    }

    /// # safety
    /// vector index is safe due to pre total population
    pub fn get_at(&self, p: Point) -> &Option<T> {
        &self.vec[self.point_to_index(p)]
    }
}

pub struct Game {
    pub grid: Grid<Rc<RefCell<dyn GameObject>>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            grid: Grid::new(4, 3),
        }
    }

    pub fn insert_at(&mut self, game_object: Rc<RefCell<dyn GameObject>>, p: Point) {
        self.grid.insert_at(game_object, p);
    }

    pub fn apply(&self, game_action: GameAction) {
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
                    target.borrow_mut().take_damage(damage);
                    println!(
                        "{0} takes {1} damage and now has {2} hp",
                        target.borrow().get_name(),
                        damage,
                        target.borrow().get_hp()
                    );
                }
            }
            GameAction::Move { target, vector: _ } => println!("moving"),
            GameAction::None => println!("do nothing"),
        }
    }
}
