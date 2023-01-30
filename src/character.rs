use crate::dicepool::DicePool;
use crate::dice::Dice;
use crate::ability_scores::AbilityScores;
use crate::ancestry::Ancestry;
use crate::character_error::CharacterError;
use crate::class::Class;

use names::Generator;

pub type MoneyType = i32;
pub type LevelType = u8;
pub type ExpType = u32;
pub type HpType = i32;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub ancestry: Ancestry,
    pub class: Class,
    pub ability_scores: AbilityScores,
    pub money: MoneyType,
    pub level: LevelType,
    pub exp: ExpType    
}

impl Character {    
    pub fn new(name: String,
	       ancestry: Ancestry,
	       class: Class,
	       ability_scores: AbilityScores,
	       money: MoneyType) -> Result<Self, CharacterError> {
	// put restrictions in the type system
	let character = Character {
	    name,
	    ancestry,
	    class,
	    ability_scores,
	    money,
	    level: 1,
	    exp: 0
	};
	if character.is_valid() {
	    Ok(character)
	} else  {
	    Err(CharacterError::RestrictionError)
	}
    }

    fn is_valid(&self) -> bool {
	true
    }
    
    fn gen_name() -> String {
	let mut name_generator = Generator::default();
	name_generator.next().unwrap()
    }

    fn gen_money() -> MoneyType {
	DicePool::new(3, Dice::D6).dice_roll_sum().sum as MoneyType * 10
    }
    
    pub fn gen() -> Self {
	loop {
	    let name = Self::gen_name();
	    let ability_scores = AbilityScores::gen();
	    let ancestry = Ancestry::gen();
	    let class = Class::gen();
	    let money = Self::gen_money();
	    match Self::new(name, ancestry, class, ability_scores, money) {
		Ok(c) => break c,
		Err(_) => ()
	    }
	}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lots_of_chars_restrictions_class() {
	for _ in 0..100 {
	    let c = Character::gen();
	    if c.ancestry == Ancestry::Dwarf {
		assert!(c.class != Class::Thief)
	    }		
	}
    }

    #[test]
    fn lots_of_chars_restrictions_abilities() {
	for _ in 0..100 {
	    let c = Character::gen();
	    if c.ancestry == Ancestry::Dwarf {
		assert!(c.ability_scores.con.1 >= 8)		
	    }
	}
    }
}
