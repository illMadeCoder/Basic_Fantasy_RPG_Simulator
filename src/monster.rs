use crate::agent::Agent;
use crate::Action;
use crate::ActionType;
use crate::Attackable;
use crate::Dice;
use crate::DicePool;
use crate::HasAC;
use crate::HasHP;
use crate::HasName;

pub struct Monster {
    pub name: String,
    pub ac: i32,
    pub hit_dice: u8,
    pub no_of_attacks: u8,
    pub damage: DicePool,
    pub max_hp: i32,
    pub hp: i32,
}

impl Agent for Monster {}

impl HasAC for Monster {
    fn ac(&self) -> i32 {
        self.ac
    }
}

impl HasHP for Monster {
    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_max_hp(&self) -> i32 {
        self.max_hp
    }

    fn set_hp(&mut self, x: i32) {
        self.hp = x;
    }
}

impl HasName for Monster {
    fn name(&self) -> &str {
        &self.name
    }
}
