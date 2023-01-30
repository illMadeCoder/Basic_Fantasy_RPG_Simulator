use rand::Rng;
use crate::character_error::CharacterError;
use crate::ability::Ability;
use crate::ability_scores::AbilityScores;


#[derive(Debug, PartialEq)]
pub enum Class {
    Cleric,
    Fighter,       
    MagicUser,
    Thief    
}
const CLASS_VARIANT_COUNT : u8 = 4;

impl Class {
    pub fn prime_requisite(self) -> Ability {
	match self {
	    Class::Cleric => Ability::Wis,
	    Class::Fighter => Ability::Str,
	    Class::MagicUser => Ability::Int,
	    Class::Thief => Ability::Dex,
	}
    }

    fn valid(self, ability_scores : AbilityScores) -> bool {
	ability_scores.get(self.prime_requisite()) >= 9
    }
    
    pub fn gen() -> Self {
    	let mut rng = rand::thread_rng();
	
	rng.gen_range(0..=CLASS_VARIANT_COUNT-1)
	    .try_into()
	    .expect("while generating class, gen_range out of range check CLASS_VARIANT_COUNT")
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
