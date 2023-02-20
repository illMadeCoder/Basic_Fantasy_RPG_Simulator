use crate::dicepool::DicePool;

pub struct Action<'a> {
    action_type: ActionType<'a>,
}

#[derive(Debug)]
pub enum ActionResult {
    MeleeAttack {
        hit: bool,
        attack_roll: i32,
        damage_roll: i32,
    },
}

impl Action<'_> {
    pub fn new(action_type: ActionType) -> Action {
        Action { action_type }
    }

    pub fn invoke(&mut self) -> ActionResult {
        match &mut self.action_type {
            ActionType::MeleeAttack {
                attack,
                damage,
                target,
            } => {
                let attack_roll = attack.dice_roll_sum().sum;
                let damage_roll = damage.dice_roll_sum().sum;
                if target.attack(attack_roll, damage_roll) {
                    ActionResult::MeleeAttack {
                        hit: true,
                        attack_roll,
                        damage_roll,
                    }
                } else {
                    ActionResult::MeleeAttack {
                        hit: false,
                        attack_roll,
                        damage_roll,
                    }
                }
            }
            ActionType::MissileAttack => todo!(),
        }
    }
}

pub trait HasAC {
    fn ac(&self) -> i32;
}

pub trait HasHP {
    fn get_max_hp(&self) -> i32;
    fn get_hp(&self) -> i32;
    fn set_hp(&mut self, damage: i32);
}

pub trait HasName {
    fn name(&self) -> &str;
}

pub trait Attackable: HasName {
    fn is_hit(&self, hit_roll: i32) -> bool;
    fn apply_damage(&mut self, damage: i32);
    fn attack(&mut self, hit_roll: i32, damage: i32) -> bool {
        //        println!("{0} attacks {1}", c.name, m.name);
        if self.is_hit(hit_roll) {
            //            println!("{0} {1} {2}", m.name, hit_or_miss_text, c.name);
            self.apply_damage(damage);
            return true;
        }
        false
    }
}

impl<T: HasAC + HasHP + HasName> Attackable for T {
    fn is_hit(&self, hit_roll: i32) -> bool {
        hit_roll >= self.ac()
    }

    fn apply_damage(&mut self, damage: i32) {
        self.set_hp(self.get_hp() - damage)
    }

    fn attack(&mut self, hit_roll: i32, damage: i32) -> bool {
        //println!("{0} attacks {1}", c.name, m.name);
        if self.is_hit(hit_roll) {
            //println!("{0} {1} {2}", m.name, hit_or_miss_text, c.name);
            self.apply_damage(damage);
            true
        } else {
            false
        }
    }
}

pub enum ActionType<'a> {
    MeleeAttack {
        attack: DicePool,
        damage: DicePool,
        target: &'a mut dyn Attackable,
    },
    MissileAttack,
}

// impl ActionType {
//     fn invoke(&self) {
//         match self {
//             ActionType::MeleeAttack => todo!(),
//             ActionType::RangedAttack => todo!(),
//         }
//     }
// }
