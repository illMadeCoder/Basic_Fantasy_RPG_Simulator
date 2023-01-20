#[derive(Debug)]
pub enum Species {
    Dwarf = 1,
    Elf,       
    Halfling,
    Human
}

struct Equipment {}
impl Species {
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
pub enum CharacterError {
    SpeciesError,
    ClassError
}

impl TryFrom<u8> for Species {
    type Error = CharacterError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
	match value {
	    x if x == Species::Dwarf as u8 => Ok(Species::Dwarf),
	    x if x == Species::Elf as u8 => Ok(Species::Elf),
	    x if x == Species::Halfling as u8 => Ok(Species::Halfling),
	    x if x == Species::Human as u8 => Ok(Species::Human),
	    _ => Err(CharacterError::SpeciesError)
	}
    }
}

#[derive(Debug)]
pub enum Class {
    Cleric = 1,
    Fighter,       
    MagicUser,
    Thief    
}

impl TryFrom<u8> for Class {
    type Error = CharacterError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
	match value {
	    x if x == Class::Cleric as u8 => Ok(Class::Cleric),
	    x if x == Class::Fighter as u8 => Ok(Class::Fighter),
	    x if x == Class::MagicUser as u8 => Ok(Class::MagicUser),
	    x if x == Class::Thief as u8 => Ok(Class::Thief),
	    _ => Err(CharacterError::ClassError)
	}
    }
}

pub type AbilityScoreType = i64;
pub type HpType = i64;

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
	    Ability::Str => ability_scores.str,
	    Ability::Int => ability_scores.int,
	    Ability::Wis => ability_scores.wis,
	    Ability::Dex => ability_scores.dex,
	    Ability::Con => ability_scores.con,
	    Ability::Cha => ability_scores.cha
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

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub species: Species,
    pub class: Class,
    pub ability_scores: AbilityScores
}

impl Character {
    pub fn new(name: String, species: Species, class: Class, ability_scores: AbilityScores) -> Character {
	Character {
	    name,
	    species,
	    class,
	    ability_scores
	}
    }
}
