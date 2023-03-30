use super::Point;

pub struct Actor {
    pub position: Point,
    pub hp: i32,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Idle,
    Move(usize)
}

impl Actor {
    fn action(&self) -> Action {
	Action::Idle
    }
}

pub struct Game {
    actors: Vec<Actor>,
    turn: usize
}

impl Game {
    pub fn new(actors: Vec<Actor>) -> Game {
	Game {
	    actors,
	    turn: 0
	}
    }
    
    pub fn step(&mut self) {
	let next_action = self.action(self.turn);
	self.inc_turn();
	self.apply(next_action);
    }

    fn inc_turn(&mut self) {
	self.turn += 1;
	if self.turn >= self.actors.len() {
	    self.turn = 0;
	}
    }
    
    fn action(&self, turn: usize) -> Action {
	if self.actors.len() == 0 {
	    panic!("No actors to get action from");
	}

	if let Some(actor) = self.actors.get(turn) {
	    actor.action()
	} else {
	    panic!("No actor for turn {}", turn);
	}
    }

    fn apply(&mut self, action: Action) {
	println!("{:?}", action);
    }
}
