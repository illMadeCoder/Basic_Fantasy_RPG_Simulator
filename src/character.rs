use names::Generator;
use crate::dicepool::DicePool;
use crate::dice::Dice;
use rand::Rng;

#[derive(Debug)]
pub enum CharacterError {
    AncestryError(u8),
    ClassError(u8),
    RestrictionError,
}

#[derive(Debug, PartialEq)]
pub enum Ancestry {
    Dwarf,
    Elf,       
    Halfling,
    Human
}
const ANCESTRY_VARIANT_COUNT : u8 = 4;

impl TryFrom<u8> for Ancestry {
    type Error = CharacterError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
	match value {
	    x if x == Self::Dwarf as u8 => Ok(Self::Dwarf),
	    x if x == Self::Elf as u8 => Ok(Self::Elf),
	    x if x == Self::Halfling as u8 => Ok(Self::Halfling),
	    x if x == Self::Human as u8 => Ok(Self::Human),
	    x => Err(CharacterError::AncestryError(x))
	}
    }
}

impl Ancestry {
    fn gen() -> Ancestry {
	let mut rng = rand::thread_rng();
	//valid_abilities(ability_scores);
	rng.gen_range(0..=ANCESTRY_VARIANT_COUNT-1).try_into().expect("Ancestry::gen() failed out of range")
    }
    
    fn valid_class(&self, class: &Class) -> bool {
	match &self {
	    Self::Dwarf => match class {
		Class::Cleric => true,
		Class::Fighter => true,
		Class::MagicUser => false,
		Class::Thief => true
	    },
	    Self::Elf => match class {
		Class::Cleric => true,
		Class::Fighter => true,
		Class::MagicUser => true,
		Class::Thief => true
	    },
	    Self::Halfling => match class {
		Class::Cleric => true,
		Class::Fighter => true,
		Class::MagicUser => false,
		Class::Thief => true
	    },
	    Self::Human => match class {
		Class::Cleric => true,
		Class::Fighter => true,
		Class::MagicUser => true,
		Class::Thief => true
	    },
	}	
    }

    fn valid_ability_scores(&self, ability_scores: &AbilityScores) -> bool {
	self.valid_ability_score(&ability_scores.cha) &&
	    self.valid_ability_score(&ability_scores.con) &&
	    self.valid_ability_score(&ability_scores.dex) &&
	    self.valid_ability_score(&ability_scores.int) &&
	    self.valid_ability_score(&ability_scores.str) &&
	    self.valid_ability_score(&ability_scores.wis)
    }

    fn valid_ability_score(&self, ability: &Ability) -> bool {
	match self {
	    Ancestry::Dwarf => match *ability {
		Ability::Con(v) => v >= 9,
		Ability::Cha(v) => v <= 17,
		_ => true
	    },
	    Ancestry::Elf => match *ability {		
		Ability::Int(v) => v >= 9,
		Ability::Con(v) => v <= 17,
		_ => true
	    },
	    Ancestry::Halfling => match *ability {
		Ability::Dex(v) => v >= 9,
		Ability::Str(v) => v <= 17,
		_ => true
	    },
	    Ancestry::Human => true
	}
    }
}

#[derive(Debug, PartialEq)]
pub enum Class {
    Cleric,
    Fighter,       
    MagicUser,
    Thief    
}
const CLASS_VARIANT_COUNT : u8 = 4;

impl Class {
    fn gen() -> Self {
    	let mut rng = rand::thread_rng();
	rng.gen_range(0..=CLASS_VARIANT_COUNT-1).try_into().unwrap()
    } 
}

impl TryFrom<u8> for Class {
    type Error = CharacterError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
	match value {
	    x if x == Self::Cleric as u8 => Ok(Self::Cleric),
	    x if x == Self::Fighter as u8 => Ok(Self::Fighter),
	    x if x == Self::MagicUser as u8 => Ok(Self::MagicUser),
	    x if x == Self::Thief as u8 => Ok(Self::Thief),
	    x => Err(CharacterError::ClassError(x))
	}
    }
}

pub type AbilityScore = i32;

pub type HpType = i32;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Ability {
    Str(AbilityScore),
    Int(AbilityScore),
    Wis(AbilityScore),
    Dex(AbilityScore),
    Con(AbilityScore),
    Cha(AbilityScore)
}

impl Ability {
    fn get(&self) -> AbilityScore {
	match *self {
	    Self::Str (v) => v,
	    Self::Int (v) => v,
	    Self::Wis (v) => v,
	    Self::Dex (v) => v,
	    Self::Con (v) => v,
	    Self::Cha (v) => v,
	}
    }    

    // fn modifier(self, ability_scores: AbilityScores) -> i32 {
    // 	match self.access(ability_scores) {
    // 	    x if x <= 3 => -3,
    // 	    x if x <= 5 => -2,
    // 	    x if x <= 8 => -1,
    // 	    x if x <= 12 => 0,
    // 	    x if x <= 15 => 1,
    // 	    x if x <= 17 => 2,
    // 	    x if x >= 18 => 3,
    // 	    _ => 0
    // 	}
    // }
}


#[derive(Debug)]
pub struct AbilityScores {
    pub str: Ability,
    pub int: Ability,
    pub wis: Ability,
    pub dex: Ability,
    pub con: Ability,
    pub cha: Ability,
}

impl AbilityScores {    
    fn ability_score_dicepool() -> DicePool {
	DicePool::new(3, Dice::D6)
    }

    fn gen_ability_score() -> AbilityScore {	
	Self::ability_score_dicepool().dice_roll_sum().sum as AbilityScore
    }
    
    fn gen(ancestry: &Ancestry) -> Self {
	// array of 6
	let rolls = Self::gen_ability_score();
	// determine what best fits class
	// randomly choose a race that meets requirement
	AbilityScores {
	    str: Ability::Str(Self::gen_ability_score()),
	    dex: Ability::Dex(Self::gen_ability_score()),
	    int: Ability::Int(Self::gen_ability_score()),
	    wis: Ability::Wis(Self::gen_ability_score()),
	    cha: Ability::Cha(Self::gen_ability_score()),
	    con: Ability::Con(Self::gen_ability_score()),
	}
    }
 }

type MoneyType = i32;

type LevelType = u8;

type ExpType = u32;

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
	if ancestry.valid_class(&class) && ancestry.valid_ability_scores(&ability_scores) {
	    Ok(Self {
		name,
		ancestry,
		class,
		ability_scores,
		money,
		level: 1,
		exp: 0
	    })

	} else  {
	    Err(CharacterError::RestrictionError)
	}
    }
    
    pub fn gen_name() -> String {
	let mut name_generator = Generator::default();
	name_generator.next().unwrap()
    }

    pub fn gen_money() -> MoneyType {
	DicePool::new(3, Dice::D6).dice_roll_sum().sum as MoneyType * 10
    }
    
    pub fn gen() -> Self {
	loop {
	    let name = Self::gen_name();
	    let ancestry = Ancestry::gen();
	    let ability_scores = AbilityScores::gen(&ancestry);
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
		assert!(c.ability_scores.con.get() >= 8)		
	    }
	}
    }
}
