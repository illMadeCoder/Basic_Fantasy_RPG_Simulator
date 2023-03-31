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
        fn actor_attack_actor(game: &mut Game, actor_id: usize, actor_id_t: usize) {
            if actor_distance(game.get_actor(actor_id), game.get_actor(actor_id_t)) <= 1 {
                damage_actor(game.get_actor_mut(actor_id_t), 1);
                let actor_a = game.get_actor(actor_id);
                let actor_b = game.get_actor(actor_id_t);
                println!(
                    "{} attacking {}, they now have {} hp",
                    actor_a.name, actor_b.name, actor_b.hp
                );
            } else {
                println!(
                    "{} is too far away to attack {}",
                    game.get_actor(actor_id).name,
                    game.get_actor(actor_id_t).name
                );
            }
        }

        fn damage_actor(actor: &mut Actor, damage: i32) {
            actor.hp -= damage;
        }
        // end mutators

        println!("{:?}", action);

        match action {
            Action::Idle(actor_id) => (),

            Action::Move(actor_id, direction) => move_actor(self, actor_id, direction),

            Action::Attack(actor_id, actor_id_t) => actor_attack_actor(self, actor_id, actor_id_t),
        }
    }
}
