use crate::game_mod::{Actor, Point};

use super::{Direction, Game};

#[derive(Debug)]
pub enum Action {
    Idle(usize),
    Move(usize, Direction),
    Attack(usize, usize),
}

impl Action {
    pub fn execute(self, game: &mut Game) {
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

        println!("{:?}", self);

        match self {
            Action::Idle(actor_id) => (),

            Action::Move(actor_id, direction) => move_actor(game, actor_id, direction),

            Action::Attack(actor_id, actor_id_t) => actor_attack_actor(game, actor_id, actor_id_t),
        }
    }
}
