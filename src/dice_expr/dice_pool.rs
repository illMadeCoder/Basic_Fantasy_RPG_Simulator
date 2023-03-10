use super::dice::{Dice, DiceError, DiceNumType};
use std::convert::From;
use std::str::FromStr;

#[derive(Debug)]
pub enum DicePoolError {
    DiceError(DiceError),
    DicePoolParseError(String),
    ParseIntError(std::num::ParseIntError),
}

impl std::error::Error for DicePoolError {}

impl std::fmt::Display for DicePoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DicePoolError::DiceError(e) => e.fmt(f),
            DicePoolError::DicePoolParseError(s) => write!(f, "DicePoolParseError: {}", s),
            DicePoolError::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl From<std::num::ParseIntError> for DicePoolError {
    fn from(e: std::num::ParseIntError) -> Self {
        DicePoolError::ParseIntError(e)
    }
}

impl From<DiceError> for DicePoolError {
    fn from(e: DiceError) -> Self {
        DicePoolError::DiceError(e)
    }
}

#[derive(Debug)]
pub struct DicePool(Vec<Dice>);

impl From<Vec<Dice>> for DicePool {
    fn from(dicepool: Vec<Dice>) -> Self {
        DicePool(dicepool)
    }
}

impl TryFrom<Vec<DiceNumType>> for DicePool {
    type Error = DicePoolError;

    fn try_from(dicepool_vec: Vec<DiceNumType>) -> Result<Self, DicePoolError> {
        Ok(DicePool(
            dicepool_vec
                .into_iter()
                .map(Dice::try_from)
                .collect::<Result<Vec<Dice>, DiceError>>()?,
        ))
    }
}

#[derive(Debug)]
pub struct DiceRollSum {
    pub dice: Vec<Dice>,
    pub roll: Vec<u8>,
    pub sum: i32,
}

impl DicePool {
    pub fn new(quantity: usize, dice: Dice) -> DicePool {
        DicePool(vec![dice; quantity])
    }

    pub fn roll(&self) -> Vec<DiceNumType> {
        self.0.iter().map(|&x| x.roll()).collect()
    }

    pub fn dice_roll_sum(&self) -> DiceRollSum {
        let dice = self.0.clone();
        let roll = self.roll();
        let sum = roll.iter().cloned().map(i32::from).sum();
        DiceRollSum { dice, roll, sum }
    }
}

impl FromStr for DicePool {
    type Err = DicePoolError;

    fn from_str(s: &str) -> Result<DicePool, Self::Err> {
        let (quantity_str, dice_str) = s
            .split_once('d')
            .ok_or_else(|| (DicePoolError::DicePoolParseError(String::from(s))))?;
        let quantity = quantity_str.parse::<usize>()?;
        let dice = dice_str.parse::<Dice>()?;

        Ok(DicePool::from(vec![dice; quantity]))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_dicepool_ok() {
        let dicepool = "3d8".parse::<DicePool>();
        assert!(dicepool.is_ok());
    }

    #[test]
    fn parse_dicepool_err() {
        let dicepool = "8".parse::<DicePool>();
        assert!(dicepool.is_err());
    }

    #[test]
    fn parse_dicepool_roll_and_sum() {
        let dicepool = DicePool::new(3, Dice::D8);
        let subject = dicepool.dice_roll_sum();
        assert!(subject.sum == 24);
    }

    #[test]
    fn parse_dicepool_parse_int_error() {
        assert!("ad8".parse::<DicePool>().is_err())
    }
}
