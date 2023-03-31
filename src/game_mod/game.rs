use super::{Direction, Point};

pub struct Actor {
    pub name: String,
    pub position: Point,
    pub hp: i32,
    pub behavior: Behavior,
}

pub enum Behavior {
    Idle,
    Wonder,
    Agressive,
    Player
}

#[derive(Debug)]
enum Action {
    Idle,
    Move(usize, Direction),
    Attack(usize, usize),
}

impl Actor {
    fn next_action(&self, actor_id: usize, game: &Game) -> Action {
        match self.behavior {
            Behavior::Idle => Action::Idle,
	    
            Behavior::Wonder => Action::Move(actor_id, Direction::rnd()),
	    
            Behavior::Agressive => Action::Attack(
                actor_id,
                game.actors
                    .iter()
                    .enumerate()
                    .find(|&x| x.0 != actor_id)
                    .expect("can't find another to attack")
                    .0,
            ),
	    
	    Behavior::Player => {
		let stdin = std::io::stdin();
		let mut buf = String::new();
		stdin.read_line(&mut buf).unwrap();
		let trimmed = buf.trim().to_string();
		let mut split = trimmed.split(' ');
		let action = split.next().unwrap();
		if action == "move" {
		    let dir_str = split.next().unwrap();
		    let direction: Direction = dir_str.parse().unwrap();
		    Action::Move(actor_id, direction)
		} else if action == "attack" {
		    let target_str = split.next().unwrap();
		    let target: usize = target_str.parse().unwrap();
		    Action::Attack(actor_id, target)
		} else {
		    Action::Idle
		}
	    }
        }
    }
}

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

    fn get_actor_mut(&mut self, actor_id: usize) -> &mut Actor {
	self.actors.get_mut(actor_id).unwrap_or_else(|| panic!("actor {actor_id} not found"))
    }

    fn get_actor(&self, actor_id: usize) -> &Actor {
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
	// mutators
	// // physics
	fn displace_actor(actor: &mut Actor, point: Point) {
	    actor.position.set_to(point);
	}
	
	fn move_actor(game: &mut Game, actor_id: usize, direction: Direction) {
	    let actor = game.get_actor_mut(actor_id);
	    let new_position = actor.position + direction.into();
	    displace_actor(actor, new_position);
            println!("{} new position {:?}", actor.name, actor.position);
        }

	fn actor_distance(actor_a: &Actor, actor_b: &Actor) -> i32 {
	    let p1 = actor_a.position;
	    let p2 = actor_b.position;
	    // TODO: Blah, this is ugly
	    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32).sqrt() as i32
	}

	// // combat
	fn actor_attack_actor(game: &mut Game, actor_id_a: usize, actor_id_b: usize) {
	    if actor_distance(game.get_actor(actor_id_a), game.get_actor(actor_id_b)) <= 1 {
		damage_actor(game.get_actor_mut(actor_id_b), 1);
		let actor_a = game.get_actor(actor_id_a);
		let actor_b = game.get_actor(actor_id_b);
		println!(
		    "{} attacking {}, they now have {} hp",
		    actor_a.name, actor_b.name, actor_b.hp
		);
	    } else {
		println!("{} is too far away to attack {}", game.get_actor(actor_id_a).name, game.get_actor(actor_id_b).name);
	    }
        }

	fn damage_actor(actor: &mut Actor, damage: i32) {
	    actor.hp -= damage;
	}
	// end mutators
	
        println!("{:?}", action);

        match action {
            Action::Idle => (),
	    
            Action::Move(actor_id, direction) => {
                move_actor(self, actor_id, direction)
            }
	    
            Action::Attack(actor_id_a, actor_id_b) => {
                actor_attack_actor(self, actor_id_a, actor_id_b)
            }
        }
    }
}
