use crate::action::Action;
use crate::dicepool::DicePool;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub properties: Vec<ItemComponents>,
}

#[derive(Debug)]
pub enum MeleeWeaponSize {
    S,
    M,
    L,
}

#[derive(Debug)]
pub struct MissileRange {
    short: i32,
    medium: i32,
    long: i32,
}

pub struct ItemAction {}

#[derive(Debug)]
pub enum ItemComponents {
    MeleeWeapon {
        damage: DicePool,
        size: MeleeWeaponSize,
    },
    MissileWeapon {
        damage: DicePool,
        missile_range_mod: MissileRange,
    },
    Weight {
        weight: f32,
    },
    Worth {
        gold: i32,
    },
    Armor {
        ac_base: i32,
    },
    Shield {
        ac_mod: i32,
    },
}

// impl ItemComponents {
//     fn get_actions(&self) -> Vec<Action> {
//         match self {
//             ItemComponents::MeleeWeapon { damage, size } => vec![Action::MeleeAttack],
//             ItemComponents::MissileWeapon {
//                 damage,
//                 missile_range_mod,
//             } => vec![Action::MissileAttack],
//             ItemComponents::Weight { weight } => todo!(),
//             ItemComponents::Worth { gold } => todo!(),
//             ItemComponents::Armor { ac_base } => todo!(),
//             ItemComponents::Shield { ac_mod } => todo!(),
//         }
//     }
// }
