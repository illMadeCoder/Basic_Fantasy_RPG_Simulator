use std::collections::HashSet;

pub trait Actor {
    fn potential_set(&self) -> HashSet<Potential>;
}

#[derive(PartialEq, Eq, Hash)]
pub enum Potential {
    Attack,
    Move,
}
