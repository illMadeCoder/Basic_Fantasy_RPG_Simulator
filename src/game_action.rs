use crate::{dicepool::DicePool, point::Point, GameObject};

// pub trait HasAC {
//     fn ac(&self) -> i32;
// }

// pub trait HasHP {
//     fn get_max_hp(&self) -> i32;
//     fn get_hp(&self) -> i32;
//     fn set_hp(&mut self, damage: i32);
// }

// pub trait Attackable {
//     fn is_hit(&self, hit_roll: i32) -> bool;
//     fn apply_damage(&mut self, damage: i32);
//     fn attack(&mut self, hit_roll: i32, damage: i32) -> bool {
//         if self.is_hit(hit_roll) {
//             self.apply_damage(damage);
//             return true;
//         }
//         false
//     }
// }

// impl<T: HasAC + HasHP + GameObject> Attackable for T {
//     fn is_hit(&self, hit_roll: i32) -> bool {
//         hit_roll >= self.ac()
//     }

//     fn apply_damage(&mut self, damage: i32) {
//         self.set_hp(self.get_hp() - damage)
//     }

//     fn attack(&mut self, hit_roll: i32, damage: i32) -> bool {
//         if self.is_hit(hit_roll) {
//             self.apply_damage(damage);
//             true
//         } else {
//             false
//         }
//     }
// }

// pub trait AsPoint {
//     fn as_point(&mut self) -> &mut Point;
// }

// pub trait Moveable {
//     fn displace(&mut self, vector: Point);
// }

// impl<T: AsPoint> Moveable for T {
//     fn displace(&mut self, vector: Point) {
//         let p = self.as_point();
//         *p = *p + vector;
//     }
// }

// pub enum GameActionType<'a> {
//     MeleeAttack {
//         attack: DicePool,
//         damage: DicePool,
//         target: &'a dyn Attackable,
//     },
//     MissileAttack,
//     Move {
//         vector: Point,
//         target: &'a dyn Moveable,
//     },
// }

#[derive(Debug)]
pub enum GameAction {
    MeleeAttack { target: usize },
    Move { target: usize, vector: Point },
    None,
}

// pub struct GameAction<'a> {
//     action_type: GameActionType<'a>,
// }

// #[derive(Debug)]
// pub enum GameActionResult {
//     MeleeAttack {
//         hit: bool,
//         attack_roll: i32,
//         damage_roll: i32,
//     },
//     Move,
//     None,
// }

// impl GameAction<'_> {
//     pub fn new(action_type: GameActionType) -> GameAction {
//         GameAction { action_type }
//     }

//     pub fn invoke(&mut self) -> GameActionResult {
//         match &mut self.action_type {
//             GameActionType::MeleeAttack {
//                 attack,
//                 damage,
//                 target,
//             } => {
//                 let attack_roll = attack.dice_roll_sum().sum;
//                 let damage_roll = damage.dice_roll_sum().sum;
//                 if target.attack(attack_roll, damage_roll) {
//                     GameActionResult::MeleeAttack {
//                         hit: true,
//                         attack_roll,
//                         damage_roll,
//                     }
//                 } else {
//                     GameActionResult::MeleeAttack {
//                         hit: false,
//                         attack_roll,
//                         damage_roll,
//                     }
//                 }
//             }
//             GameActionType::MissileAttack => todo!(),
//             GameActionType::Move { target, vector } => {
//                 target.displace(*vector);
//                 GameActionResult::Move
//             }
//         }
//     }
// }
