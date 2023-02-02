use crate::ability_score_set::AbilityScoreSet;
use crate::ancestry::Ancestry;
use crate::character_error::CharacterError;
use crate::class::Class;
use crate::dice::Dice;
use crate::dicepool::DicePool;
use crate::item::Weapon;

use names::Generator;

pub type Money = i32;
pub type Level = u8;
pub type ExpType = u32;
pub type HpType = i32;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub ancestry: Ancestry,
    pub class: Class,
    pub ability_score_set: AbilityScoreSet,
    pub money: Money,
    pub level: Level,
    pub exp: ExpType,
    pub weapon: Option<Weapon>,
    pub ac: i32,
    pub hp: i32,
    pub max_hp: u32,
}

impl Character {
    pub fn new(
        name: String,
        ancestry: Ancestry,
        class: Class,
        ability_score_set: AbilityScoreSet,
        money: Money,
    ) -> Result<Self, CharacterError> {
        if Character::is_valid(&ability_score_set, &ancestry, &class) {
            Ok(Character {
                name,
                ancestry,
                class,
                ability_score_set,
                money,
                level: 1,
                exp: 0,
                weapon: None,
                ac: 10,
                max_hp: 8,
                hp: 8,
            })
        } else {
            Err(CharacterError::InvalidCharacterError)
        }
    }

    fn is_valid(ability_score_set: &AbilityScoreSet, ancestry: &Ancestry, class: &Class) -> bool {
        let ancestry_supports_class = ancestry.supports_class(class);
        let ancestry_supports_ability_score_set =
            ancestry.supports_ability_score_set(ability_score_set);
        let class_supports_ability_score_set = class.supports_ability_score_set(ability_score_set);

        ancestry_supports_class
            && ancestry_supports_ability_score_set
            && class_supports_ability_score_set
    }

    fn gen_name() -> String {
        let mut name_generator = Generator::default();
        name_generator.next().unwrap()
    }

    fn gen_money() -> Money {
        DicePool::new(3, Dice::D6).dice_roll_sum().sum as Money * 10
    }

    pub fn gen() -> Self {
        loop {
            let name = Self::gen_name();
            let ability_score_set = AbilityScoreSet::gen();
            let ancestry = Ancestry::gen();
            let class = Class::gen();
            let money = Self::gen_money();
            match Self::new(name, ancestry, class, ability_score_set, money) {
                Ok(c) => break c,
                Err(_) => (),
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
                assert!(c.ability_score_set.con.1 >= 8)
            }
        }
    }
}
