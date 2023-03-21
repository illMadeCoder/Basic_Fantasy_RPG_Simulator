use crate::dice_expr_mod::{Dice, DicePool};

#[derive(Debug)]
pub struct Monster {
    pub name: String,
    pub ac: i32,
    pub hit_dice: u8,
    pub no_of_attacks: u8,
    pub damage: DicePool,
    pub hp: i32,
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
        }
    }
}
