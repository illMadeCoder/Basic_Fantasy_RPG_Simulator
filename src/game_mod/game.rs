use crate::game_mod::Action;

use super::{Direction, Point, Actor};

pub struct Game {
    pub actors: Vec<Actor>,
    turn: usize,
}

impl Game {
    pub fn new(actors: Vec<Actor>) -> Game {
        Game { actors, turn: 0 }
    }

    pub fn step(&mut self) {
	fn inc_turn(mut turn: usize, last_turn: usize) -> usize {
	    turn += 1;
            if turn > last_turn {
		0
            } else {
		turn
	    }
	}
	
        let next_action = self.fetch_actor_action(self.turn);
        self.turn = inc_turn(self.turn, self.actors.len()-1);
        self.process_action(next_action);
    }

    pub fn get_actor_mut(&mut self, actor_id: usize) -> &mut Actor {
	self.actors.get_mut(actor_id).unwrap_or_else(|| panic!("actor {actor_id} not found"))
    }

    pub fn get_actor(&self, actor_id: usize) -> &Actor {
	self.actors.get(actor_id).unwrap_or_else(|| panic!("actor {actor_id} not found"))
    }

    // TODO: view shouldn't have pub access, there should be a method to get a view
    pub fn actor_at(&self, point: Point) -> Option<&Actor> {
	self.actors.iter().find(|x| x.position == point)
    }

    fn fetch_actor_action(&self, actor_id: usize) -> Action {
        self.get_actor(actor_id).next_action(actor_id, self)
    }

    fn process_action(&mut self, action: Action) {
	action.execute(self);
    }
}
