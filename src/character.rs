use crate::ability_score_set::AbilityScoreSet;
use crate::action::{AsPoint, HasAC, HasHP, HasName};
use crate::agent::Agent;
use crate::ancestry::Ancestry;
use crate::character_error::CharacterError;
use crate::class::Class;
use crate::dice::Dice;
use crate::dicepool::DicePool;
use crate::item::Item;
use crate::point::Point;

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
    pub ac: i32,
    pub max_hp: i32,
    pub equipment: Equipment,
    pub position: Point,
}

impl AsPoint for Character {
    fn as_point(&mut self) -> &mut Point {
        &mut self.position
    }
}

impl Agent for Character {
    fn next_action<'a>(
        &'a mut self,
        attackable: &'a mut dyn crate::action::Attackable,
    ) -> crate::action::Action<'a> {
        // let a = crate::action::ActionType::MeleeAttack {
        //     attack: DicePool::new(1, Dice::D20),
        //     damage: DicePool::new(1, Dice::D8),
        //     target: attackable,
        // };

        let a = crate::action::ActionType::Move {
            target: self,
            vector: Point { x: 0, y: -1 },
        };
        crate::action::Action::new(a)
    }

    fn take_turn<'a>(&'a mut self, attackable: &'a mut dyn crate::action::Attackable) {
        let mut action = self.next_action(attackable);
        let action_result = action.invoke();
        println!("{0} attacks {1}", self.name(), attackable.name());
        println!("{:?}", action_result);
    }
}

impl HasName for Character {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasAC for Character {
    fn ac(&self) -> i32 {
        self.ac
    }
}

impl HasHP for Character {
    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_max_hp(&self) -> i32 {
        self.max_hp
    }

    fn set_hp(&mut self, x: i32) {
        self.hp = x;
    }
}

#[derive(Debug)]
pub enum Hand {
    OneHanded(Item),
    DualHanded(Item, Item),
    TwoHanded(Item),
    None,
}

#[derive(Debug)]
pub struct Equipment {
    pub hand: Hand,
    pub armor: Option<Item>,
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
                    hand: Hand::None,
                    armor: None,
                },
                level: 1,
                exp: 0,
                max_hp: 8,
                hp: 8,
                ac: 10,
                position: Point { x: 3, y: 3 },
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
        DicePool::new(3, Dice::D6).dice_roll_sum().sum as i32 * 10
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
