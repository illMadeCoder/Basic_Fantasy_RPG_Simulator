use super::ability::Ability;
use super::ability_score::AbilityScore;
use crate::dice_expr_mod::{Dice, DicePool};

#[derive(Debug)]
pub struct AbilityScoreSet {
    pub str: AbilityScore,
    pub int: AbilityScore,
    pub wis: AbilityScore,
    pub dex: AbilityScore,
    pub con: AbilityScore,
    pub cha: AbilityScore,
}

impl IntoIterator for AbilityScoreSet {
    type Item = AbilityScore;

    type IntoIter = std::array::IntoIter<AbilityScore, 6>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_arr().into_iter()
    }
}

impl AbilityScoreSet {
    fn ability_score_dicepool() -> DicePool {
        DicePool::new(3, Dice::D6)
    }

    fn into_arr(self) -> [AbilityScore; 6] {
        [self.str, self.int, self.wis, self.dex, self.con, self.cha]
    }

    pub fn get(&self, ability: Ability) -> i32 {
        match ability {
            Ability::Str => self.str.1,
            Ability::Int => self.int.1,
            Ability::Wis => self.wis.1,
            Ability::Dex => self.dex.1,
            Ability::Con => self.con.1,
            Ability::Cha => self.cha.1,
        }
    }

    fn gen_score() -> i32 {
        Self::ability_score_dicepool().dice_roll_sum().sum
    }

    pub fn gen() -> Self {
        AbilityScoreSet {
            str: AbilityScore(Ability::Str, Self::gen_score()),
            dex: AbilityScore(Ability::Dex, Self::gen_score()),
            int: AbilityScore(Ability::Int, Self::gen_score()),
            wis: AbilityScore(Ability::Wis, Self::gen_score()),
            cha: AbilityScore(Ability::Cha, Self::gen_score()),
            con: AbilityScore(Ability::Con, Self::gen_score()),
        }
    }
}
