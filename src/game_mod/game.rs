use super::{Direction, Point};

pub struct Actor {
    pub position: Point,
    pub hp: i32,
    pub behavior: Behavior,
}

pub enum Behavior {
    Idle,
    Wonder,
    Agressive,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Idle,
    Move(usize, Direction),
    Attack(usize, usize),
}

impl Actor {
    fn action(&self, actor_id: usize, game: &Game) -> Action {
        match self.behavior {
            Behavior::Idle => Action::Idle,
            Behavior::Wonder => Action::Move(actor_id, Direction::rnd()),
            Behavior::Agressive => Action::Attack(actor_id, game.actors.iter()
						  .enumerate()
						  .find(|&x| x.0 != actor_id).expect("can't find another to attack").0),
	}	    
    }
}

pub struct Game {
    actors: Vec<Actor>,
    turn: usize,
}

impl Game {
    pub fn new(actors: Vec<Actor>) -> Game {
        Game { actors, turn: 0 }
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

    fn action(&self, actor_id: usize) -> Action {
        if self.actors.len() == 0 {
            panic!("No actors to get action from");
        }

        if let Some(actor) = self.actors.get(actor_id) {
            actor.action(actor_id, self)
        } else {
            panic!("No actor for actor_id {}", actor_id);
        }
    }

    fn move_actor(&mut self, actor_id: usize, direction: Direction) {
        if let Some(actor) = self.actors.get_mut(actor_id) {
            actor.position.add_to(&(direction.into()));
            println!("{} new position {:?}", actor_id, actor.position);
        }
    }

    fn actor_attack_actor(&mut self, actor_id_a: usize, actor_id_b: usize) {
	if actor_id_a == actor_id_b {
	    panic!("actor_id {} attacking itself", actor_id_a)
	}
        if let Some(actor_b) = self.actors.get_mut(actor_id_b) {
            actor_b.hp -= 1;
	    println!("{} attacking {}, they now have {} hp", actor_id_a, actor_id_b, actor_b.hp);		
        }
	else {
	    panic!("shouldn't be here");
	}
    }

    fn apply(&mut self, action: Action) {
        println!("{:?}", action);

        match action {
            Action::Idle => (),
            Action::Move(actor_id, direction) => self.move_actor(actor_id, direction),
            Action::Attack(actor_id_a, actor_id_b) => {
                self.actor_attack_actor(actor_id_a, actor_id_b)
            }
        }
    }
}
