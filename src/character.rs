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
    fn gen(ability_scores : &AbilityScores) -> Ancestry {
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

    fn valid_ability_score(&self, ability_score: &AbilityScore) -> bool {
	match self {
	    Ancestry::Dwarf => match ability_score.0 {
		Ability::Con => ability_score.1 >= 9,
		Ability::Cha => ability_score.1 <= 17,
		_ => true
	    },
	    Ancestry::Elf => match ability_score.0  {		
		Ability::Int => ability_score.1 >= 9,
		Ability::Con => ability_score.1 <= 17,
		_ => true
	    },
	    Ancestry::Halfling => match ability_score.0 {
		Ability::Dex => ability_score.1 >= 9,
		Ability::Str => ability_score.1 <= 17,
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
    fn prime_requisite(class : Class) -> Ability {
	match class {
	    Class::Cleric => Ability::Wis,
	    Class::Fighter => Ability::Str,
	    Class::MagicUser => Ability::Int,
	    Class::Thief => Ability::Dex,
	}
    }
    fn gen(ability_scores : &AbilityScores) -> Self {	
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

pub type HpType = i32;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Ability {
    Str,
    Int,
    Wis,
    Dex,
    Con,
    Cha
}

#[derive(Debug)]
pub struct AbilityScore (Ability, i32);

impl AbilityScore {
    fn modifier(self) -> i32 {
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


#[derive(Debug)]
pub struct AbilityScores {
    pub str: AbilityScore,
    pub int: AbilityScore,
    pub wis: AbilityScore,
    pub dex: AbilityScore,
    pub con: AbilityScore,
    pub cha: AbilityScore,
}

impl AbilityScores {    
    fn ability_score_dicepool() -> DicePool {
	DicePool::new(3, Dice::D6)
    }

    fn gen_score() -> i32 {
	Self::ability_score_dicepool().dice_roll_sum().sum as i32
    }
    
    fn gen() -> Self {
	// array of 6
	let rolls = Self::gen_score();
	// determine what best fits class
	// randomly choose a race that meets requirement
	AbilityScores {
	    str: AbilityScore (Ability::Str, Self::gen_score()),
	    dex: AbilityScore (Ability::Dex, Self::gen_score()),
	    int: AbilityScore (Ability::Int, Self::gen_score()),
	    wis: AbilityScore (Ability::Wis, Self::gen_score()),
	    cha: AbilityScore (Ability::Cha, Self::gen_score()),
	    con: AbilityScore (Ability::Con, Self::gen_score())
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
	    let ability_scores = AbilityScores::gen();
	    let ancestry = Ancestry::gen(&ability_scores);
	    let class = Class::gen(&ability_scores);
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
