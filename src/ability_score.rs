use crate::ability::Ability;

pub type Score = i32;

pub type Modifier = i32;

#[derive(Debug)]
pub struct AbilityScore (pub Ability, pub Score);

impl AbilityScore {
    fn modifier(self) -> Modifier {
	match self.1 {
	    x if x <= 3 => -3,
	    x if x <= 5 => -2,
	    x if x <= 8 => -1,
	    x if x <= 12 => 0,
	    x if x <= 15 => 1,
	    x if x <= 17 => 2,
	    x if x >= 18 => 3,
	    _ => 0
	}
    }
}
