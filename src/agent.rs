use crate::Action;
use crate::ActionType;
use crate::Attackable;
use crate::Dice;
use crate::DicePool;
use crate::HasName;

pub trait Agent: HasName {
    fn next_action<'a>(&self, attackable: &'a mut dyn Attackable) -> Action<'a> {
        let a = ActionType::MeleeAttack {
            attack: DicePool::new(1, Dice::D20),
            damage: DicePool::new(1, Dice::D8),
            target: attackable,
        };
        Action::new(a)
    }
    fn take_turn<'a>(&'a self, attackable: &'a mut dyn Attackable) {
        let mut action = self.next_action(attackable);
        let action_result = action.invoke();
        println!("{0} attacks {1}", self.name(), attackable.name());
        println!("{:?}", action_result);
    }
}
