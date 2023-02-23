use super::{GameAction, GameObject};

pub struct Game<'a> {
    pub game_objects: Vec<&'a mut dyn GameObject>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            game_objects: Vec::new(),
        }
    }

    pub fn insert(&mut self, game_objects: &'a mut dyn GameObject) -> usize {
        let i = self.game_objects.len();
        self.game_objects.push(game_objects);
        i
    }

    pub fn get(&self, id: usize) -> &dyn GameObject {
        self.game_objects[id]
    }

    pub fn get_id(&self, game_object: &dyn GameObject) -> Option<usize> {
        for (id, a_game_object) in self.game_objects.iter().enumerate() {
            if game_object as *const _ == *a_game_object as *const _ {
                return Some(id);
            }
        }
        None
    }

    pub fn get_by_name(&self, name: String) -> Vec<usize> {
        self.game_objects
            .iter()
            .filter(|x| x.get_name() == name)
            .filter_map(|x| self.get_id(*x))
            .collect()
    }

    pub fn apply(&'a mut self, game_action: GameAction) {
        match game_action {
            GameAction::MeleeAttack { target } => {
                let name = self.game_objects[target].get_name();
                let hp = self.game_objects[target].get_hp();
                println!("attacking {name} with {hp}");
                self.game_objects[target].take_damage(10);
                let hp = self.game_objects[target].get_hp();
                let name = self.game_objects[target].get_name();
                println!("{name} now has {hp}");
            }
            GameAction::Move { target, vector: _ } => println!("moving {target}"),
            GameAction::None => println!("do nothing"),
        }
    }
}
