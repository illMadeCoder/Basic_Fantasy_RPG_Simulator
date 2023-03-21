use super::ability_score_set::AbilityScoreSet;
use super::ancestry::Ancestry;
use super::character_error::CharacterError;
use super::class::Class;
use super::equipment::{Equipment, HandEquipType};
use crate::dice_expr_mod::{Dice, DicePool};

use names::Generator;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub ancestry: Ancestry,
    pub class: Class,
    pub ability_score_set: AbilityScoreSet,
    pub money: i32,
    pub level: u8,
    pub exp: u32,
    pub hp: i32,
    pub equipment: Equipment,
}

impl Character {
    pub fn new(
        name: String,
        ancestry: Ancestry,
        class: Class,
        ability_score_set: AbilityScoreSet,
        money: i32,
    ) -> Result<Self, CharacterError> {
        if Character::is_valid(&ability_score_set, &ancestry, &class) {
            Ok(Character {
                name,
                ancestry,
                class,
                ability_score_set,
                money,
                equipment: Equipment {
                    hand: HandEquipType::None,
                    armor: None,
                },
                level: 1,
                exp: 0,
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

    fn gen_money() -> i32 {
        DicePool::new(3, Dice::D6).dice_roll_sum().sum * 10
    }

    pub fn gen() -> Self {
        loop {
            let name = Self::gen_name();
            let ability_score_set = AbilityScoreSet::gen();
            let ancestry = Ancestry::gen();
            let class = Class::gen();
            let money = Self::gen_money();
            if let Ok(c) = Self::new(name, ancestry, class, ability_score_set, money) {
                break c;
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
