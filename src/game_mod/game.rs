use super::{GameAction, GameObject};
use crate::{direction::DIRECTIONS, grid::Grid, point::Point};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Game {
    pub game_objects: Vec<Rc<RefCell<dyn GameObject>>>,
    pub turn_table: Vec<Weak<RefCell<dyn GameObject>>>,
    pub grid: Grid<Weak<RefCell<dyn GameObject>>>,
    pub turn: i32,
    pub over: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            game_objects: Vec::new(),
            turn_table: Vec::new(),
            grid: Grid::new(4, 4),
            turn: 0,
            over: false,
        }
    }

    pub fn take_turn(&mut self) {
        println!("turn {}", self.turn);
        // get game object whose turn it is
        // let game_object = self.turn_table[self.turn as usize];
        // get game object agent
        let stdin = std::io::stdin();
        let c = self.turn_table[0].clone();
        let m = self.turn_table[1].clone();
        if self.turn % 2 == 0 {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();
            let trimmed = buf.trim().to_string();
            let mut split = trimmed.split(' ');
            let action = split.next().unwrap();
            if action == "move" {
                let dir = split.next().unwrap();
                // .apply(GameAction::Move {
                //     target: Rc::clone(&c),
                //     direction: dir.parse().unwrap(),
                // });
            } else if action == "attack" {
                self.apply(GameAction::MeleeAttack {
                    source: c,
                    target: m,
                });
            } else {
                self.apply(GameAction::None);
            }
        } else {
            self.apply(GameAction::MeleeAttack {
                source: m,
                target: c,
            });
        }
        // if c.borrow().get_hp() <= 0 {
        //     self.over = true;
        //     println!("{} won", m.borrow().get_name());
        // } else if m.borrow().get_hp() <= 0 {
        //     self.over = true;
        //     println!("{} won", c.borrow().get_name());
        // }

        self.turn += 1;
    }

    fn surrounding(p: &Point) -> Vec<Point> {
        DIRECTIONS.iter().map(|dir| Point::from(dir) + *p).collect()
    }

    fn surrounding_game_objects(&self, p: &Point) -> Vec<&Weak<RefCell<dyn GameObject>>> {
        Game::surrounding(p)
            .iter()
            .filter_map(|s_p| self.grid.get(s_p))
            .collect()
    }

    // pub fn insert(&mut self, game_object: &Rc<RefCell<dyn GameObject>>) {
    //     let rc = Rc::clone(game_object);
    //     let position = rc.borrow().get_position();
    //     self.grid.insert(rc, &position);

    //     let rc = Rc::clone(game_object);
    //     self.turn_table.push(rc);
    // }

    pub fn insert(&mut self, game_object: Rc<RefCell<dyn GameObject>>) {
        let position = game_object.borrow().get_position();
        self.grid.insert(Rc::downgrade(&game_object), &position);
        self.turn_table.push(Rc::downgrade(&game_object));
        self.game_objects.push(game_object);
    }

    pub fn apply(&mut self, game_action: GameAction) {
        match game_action {
            GameAction::MeleeAttack { source, target } => {
                let source_position = source.upgrade().unwrap().borrow().get_position();
                if !self
                    .surrounding_game_objects(&source_position)
                    .iter()
                    .any(|x| Rc::ptr_eq(&x.upgrade().unwrap(), &target.upgrade().unwrap()))
                {
                    println!("target is not in reach");
                    return ();
                }

                println!(
                    "{0} is attacking {1}",
                    source.upgrade().unwrap().borrow().get_name(),
                    target.upgrade().unwrap().borrow().get_name()
                );

                let roll = 15;
                println!(
                    "{0} rolls to hit against AC {1}",
                    source.upgrade().unwrap().borrow().get_name(),
                    roll
                );

                if roll > target.upgrade().unwrap().borrow().get_ac() {
                    let damage = 3;
                    println!(
                        "{0} rolls {1} for damage",
                        source.upgrade().unwrap().borrow().get_name(),
                        damage
                    );
                    target.upgrade().unwrap().borrow_mut().take_damage(damage);
                    println!(
                        "{0} takes {1} damage and now has {2} hp",
                        target.upgrade().unwrap().borrow().get_name(),
                        damage,
                        target.upgrade().unwrap().borrow().get_hp()
                    );
                }
            }

            GameAction::Move { target, direction } => {
                let prev_pos = target.upgrade().unwrap().borrow().get_position();
                self.grid.remove(&prev_pos);
                target
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .displace(direction.into());
                let cur_pos = target.upgrade().unwrap().borrow().get_position();
                self.grid.insert(target, &cur_pos);
                println!("move from {:?} to {:?}", prev_pos, cur_pos);
            }

            GameAction::None => println!("do nothing"),
        }
    }
}
