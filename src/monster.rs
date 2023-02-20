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

impl Monster {
    pub fn next_action<'a>(&'a self, attackable: &'a mut dyn Attackable) -> Action {
        let a = ActionType::MeleeAttack {
            attack: DicePool::new(1, Dice::D20),
            damage: DicePool::new(1, Dice::D8),
            target: attackable,
        };
        Action::new(a)
    }

    pub fn take_turn(&self, attackable: &mut dyn Attackable) {
        let mut action = self.next_action(attackable);
        let action_result = action.invoke();
        println!("{0} attacks {1}", self.name(), attackable.name());
        println!("{:?}", action_result);
    }
}

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
