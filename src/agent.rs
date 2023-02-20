use crate::Action;
use crate::Attackable;

pub trait Agent {
    fn next_action<'a>(&self, attackable: &'a mut dyn Attackable) -> Action<'a>;
    fn take_turn<'a>(&'a self, attackable: &'a mut dyn Attackable);
}
