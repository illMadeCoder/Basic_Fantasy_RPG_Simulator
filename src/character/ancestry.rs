use super::ability::Ability;
use super::ability_score::AbilityScore;
use super::ability_score_set::AbilityScoreSet;
use super::character_error::CharacterError;
use super::class::Class;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Ancestry {
    Dwarf,
    Elf,
    Halfling,
    Human,
}
const ANCESTRY_VARIANT_COUNT: u8 = 4;

impl TryFrom<u8> for Ancestry {
    type Error = CharacterError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Dwarf as u8 => Ok(Self::Dwarf),
            x if x == Self::Elf as u8 => Ok(Self::Elf),
            x if x == Self::Halfling as u8 => Ok(Self::Halfling),
            x if x == Self::Human as u8 => Ok(Self::Human),
            x => Err(CharacterError::AncestryError(x)),
        }
    }
}

impl Ancestry {
    pub fn gen() -> Ancestry {
        let mut rng = rand::thread_rng();
        //valid_abilities(ability_score_set);
        rng.gen_range(0..=ANCESTRY_VARIANT_COUNT - 1)
            .try_into()
            .expect("Ancestry::gen() failed out of range")
    }

    pub fn supports_class(&self, class: &Class) -> bool {
        match self {
            Self::Dwarf => match class {
                Class::Cleric => true,
                Class::Fighter => true,
                Class::MagicUser => false,
                Class::Thief => true,
            },
            Self::Elf => match class {
                Class::Cleric => true,
                Class::Fighter => true,
                Class::MagicUser => true,
                Class::Thief => true,
            },
            Self::Halfling => match class {
                Class::Cleric => true,
                Class::Fighter => true,
                Class::MagicUser => false,
                Class::Thief => true,
            },
            Self::Human => match class {
                Class::Cleric => true,
                Class::Fighter => true,
                Class::MagicUser => true,
                Class::Thief => true,
            },
        }
    }

    pub fn supports_ability_score_set(&self, ability_score_set: &AbilityScoreSet) -> bool {
        self.supports_ability_score(&ability_score_set.cha)
            && self.supports_ability_score(&ability_score_set.con)
            && self.supports_ability_score(&ability_score_set.dex)
            && self.supports_ability_score(&ability_score_set.int)
            && self.supports_ability_score(&ability_score_set.str)
            && self.supports_ability_score(&ability_score_set.wis)
    }

    fn supports_ability_score(&self, ability_score: &AbilityScore) -> bool {
        match self {
            Ancestry::Dwarf => match ability_score.0 {
                Ability::Con => ability_score.1 >= 9,
                Ability::Cha => ability_score.1 <= 17,
                _ => true,
            },
            Ancestry::Elf => match ability_score.0 {
                Ability::Int => ability_score.1 >= 9,
                Ability::Con => ability_score.1 <= 17,
                _ => true,
            },
            Ancestry::Halfling => match ability_score.0 {
                Ability::Dex => ability_score.1 >= 9,
                Ability::Str => ability_score.1 <= 17,
                _ => true,
            },
            Ancestry::Human => true,
        }
    }
}
