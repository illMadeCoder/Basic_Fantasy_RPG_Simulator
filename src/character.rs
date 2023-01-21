use names::Generator;
use crate::dicepool::DicePool;
use crate::dice::Dice;
use rand::Rng;

#[derive(Debug)]
pub enum CharacterError {
    SpeciesError(u8),
    ClassError(u8)
}

struct Equipment {}

#[derive(Debug)]
pub enum Species {
    Dwarf,
    Elf,       
    Halfling,
    Human
}

impl TryFrom<u8> for Species {
    type Error = CharacterError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
	match value {
	    x if x == Self::Dwarf as u8 => Ok(Self::Dwarf),
	    x if x == Self::Elf as u8 => Ok(Self::Elf),
	    x if x == Self::Halfling as u8 => Ok(Self::Halfling),
	    x if x == Self::Human as u8 => Ok(Self::Human),
	    x => Err(CharacterError::SpeciesError(x))
	}
    }
}

impl Species {
    fn gen() -> Species {
	let mut rng = rand::thread_rng();
	rng.gen_range(0..=3).try_into().expect("Species::gen() failed out of range")
    }
    
    fn restriction(&self, class: Class) -> bool {
	match self {
	    Species::Dwarf => match class {
		Class::Cleric => false,
		Class::Fighter => false,
		Class::MagicUser => true,
		Class::Thief => false
	    },
	    Species::Elf => match class {
		Class::Cleric => false,
		Class::Fighter => false,
		Class::MagicUser => false,
		Class::Thief => false
	    },
	    Species::Halfling => match class {
		Class::Cleric => false,
		Class::Fighter => false,
		Class::MagicUser => true,
		Class::Thief => false
	    },
	    Species::Human => match class {
		Class::Cleric => false,
		Class::Fighter => false,
		Class::MagicUser => false,
		Class::Thief => false
	    },
	}	
    }
    // fn restrictions(&self, ability_scores: AbilityScores) -> bool {
    // 	match &self {
    // 	}
    // }
    // pub fn (&self, class: Class, ability_scores: Skills, equipment: Equipment) -> bool {
    // 	match self {
    // 	    Species::Dwarf => todo!(),
    // 	    Species::Elf => todo!(),
    // 	    Species::Halfling => todo!(),
    // 	    Species::Human => todo!(),
    // 	}
    // }
}

#[derive(Debug)]
pub enum Class {
    Cleric,
    Fighter,       
    MagicUser,
    Thief    
}

impl Class {
    fn gen() -> Self {
    	let mut rng = rand::thread_rng();
	rng.gen_range(0..=3).try_into().unwrap()
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

pub type AbilityScoreType = i32;
pub type HpType = i32;

pub enum Ability {
    Str = 1,
    Int,
    Wis,
    Dex,
    Con,
    Cha
}

impl Ability {
    fn access(&self, ability_scores: AbilityScores) -> AbilityScoreType {
	match &self {
	    Self::Str => ability_scores.str,
	    Self::Int => ability_scores.int,
	    Self::Wis => ability_scores.wis,
	    Self::Dex => ability_scores.dex,
	    Self::Con => ability_scores.con,
	    Self::Cha => ability_scores.cha
	}
    }
}


#[derive(Debug)]
pub struct AbilityScores {
    pub str: AbilityScoreType,
    pub int: AbilityScoreType,
    pub wis: AbilityScoreType,
    pub dex: AbilityScoreType,
    pub con: AbilityScoreType,
    pub cha: AbilityScoreType,
}

impl AbilityScores {
    
    fn ability_score_dicepool() -> DicePool {
	DicePool::new(3, Dice::D6)
    }
    
    fn gen() -> Self {	
	AbilityScores {
	    str: Self::ability_score_dicepool().dice_roll_sum().2 as AbilityScoreType,
	    dex: Self::ability_score_dicepool().dice_roll_sum().2 as AbilityScoreType,
	    int: Self::ability_score_dicepool().dice_roll_sum().2 as AbilityScoreType,
	    wis: Self::ability_score_dicepool().dice_roll_sum().2 as AbilityScoreType,
	    cha: Self::ability_score_dicepool().dice_roll_sum().2 as AbilityScoreType,
	    con: Self::ability_score_dicepool().dice_roll_sum().2 as AbilityScoreType,
	}
    }
    
}

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub species: Species,
    pub class: Class,
    pub ability_scores: AbilityScores
}

impl Character {    
    pub fn new(name: String, species: Species, class: Class, ability_scores: AbilityScores) -> Self {
	Self {
	    name,
	    species,
	    class,
	    ability_scores
	}
    }

    pub fn gen() -> Self {
	let mut name_generator = Generator::default();
	let gen_name = name_generator.next().unwrap();
	
	Self {
	    name: gen_name,
	    species: Species::gen(),
	    class: Class::gen(),
	    ability_scores: AbilityScores::gen()
	}
    }
}
