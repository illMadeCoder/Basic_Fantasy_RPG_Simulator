use crate::dice_expr::{Dice, DicePool};
use crate::point::Point;

#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub ac: i32,
    pub hit_dice: u8,
    pub no_of_attacks: u8,
    pub damage: DicePool,
    pub max_hp: i32,
    pub hp: i32,
    pub position: Point,
}

impl Monster {
    pub fn gen() -> Monster {
        Monster {
            name: "Goblin".to_string(),
            ac: 14,
            hit_dice: 1,
            no_of_attacks: 1,
            damage: DicePool::new(1, Dice::D6),
            hp: 8,
            max_hp: 8,
            position: Point { x: 4, y: 3 },
        }
    }
}

// impl AsPoint for Monster {
//     fn as_point(&mut self) -> &mut Point {
//         &mut self.position
//     }
// }

// impl Agent for Monster {
//     fn next_action<'a>(&'a self, game: &'a mut Game) -> GameAction<'a> {
//         let a = GameActionType::MeleeAttack {
//             attack: DicePool::new(1, Dice::D20),
//             damage: DicePool::new(1, Dice::D8),
//             target: &mut game.character,
//         };

//         GameAction::new(a)
//     }
//     fn take_turn<'a>(&'a self, game: &'a mut Game) {
//         let mut action = self.next_action(game);
//         let action_result = action.invoke();
//         println!("{0} attacks {1}", self.name(), game.character.name());
//         println!("{:?}", action_result);
//     }
// }

// impl HasAC for Monster {
//     fn ac(&self) -> i32 {
//         self.ac
//     }
// }

// impl HasHP for Monster {
//     fn get_hp(&self) -> i32 {
//         self.hp
//     }

//     fn get_max_hp(&self) -> i32 {
//         self.max_hp
//     }

//     fn set_hp(&mut self, x: i32) {
//         self.hp = x;
//     }
// }
