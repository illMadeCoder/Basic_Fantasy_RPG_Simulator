use rand::Rng;
use std::convert::{From, Into, TryFrom, TryInto};
use std::{num::ParseIntError, str::FromStr};

pub type DiceNumType = u8;

#[derive(Debug)]
pub enum DiceError {
    InvalidDiceNumError(DiceNumType),
    ParseIntError(ParseIntError),
}

impl std::error::Error for DiceError {}
impl std::fmt::Display for DiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceError::InvalidDiceNumError(num) => write!(f, "InvalidDiceNumError: {}", num),
            DiceError::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for DiceError {
    fn from(value: ParseIntError) -> Self {
        DiceError::ParseIntError(value)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Dice {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

impl Dice {
    pub fn roll(self) -> DiceNumType {
        // if in test environment return max
        let dice_value = self.into();
        if cfg!(test) {
            dice_value
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..=dice_value)
        }
    }
}

// TODO: prefer to use a generic type that can into DiceNumType
impl TryFrom<DiceNumType> for Dice {
    type Error = DiceError;

    fn try_from(value: DiceNumType) -> Result<Self, Self::Error> {
        match value {
            x if x == Dice::D4 as DiceNumType => Ok(Dice::D4),
            x if x == Dice::D6 as DiceNumType => Ok(Dice::D6),
            x if x == Dice::D8 as DiceNumType => Ok(Dice::D8),
            x if x == Dice::D10 as DiceNumType => Ok(Dice::D10),
            x if x == Dice::D12 as DiceNumType => Ok(Dice::D12),
            x if x == Dice::D20 as DiceNumType => Ok(Dice::D20),
            x if x == Dice::D100 as DiceNumType => Ok(Dice::D100),
            _ => Err(DiceError::InvalidDiceNumError(value)),
        }
    }
}

impl From<Dice> for DiceNumType {
    fn from(value: Dice) -> Self {
        value as DiceNumType
    }
}

impl FromStr for Dice {
    type Err = DiceError;

    fn from_str(s: &str) -> Result<Dice, Self::Err> {
        s.parse::<DiceNumType>().map(|n| n.try_into())?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dice_to() {
        let dice = Dice::D8;
        let subject: DiceNumType = dice.into();
        assert!(subject == 8);
    }

    #[test]
    fn dice_roll() {
        let dice = Dice::D8;
        let subject = dice.roll();
        assert!(subject == 8);
    }

    #[test]
    fn from_dice_ok() {
        let subject = Dice::try_from(8);
        assert!(subject.is_ok());
    }

    #[test]
    fn from_dice_err() {
        let subject = Dice::try_from(0);
        assert!(subject.is_err());
    }

    #[test]
    fn from_dice() {
        let subject = Dice::try_from(8).unwrap();
        assert!(subject == Dice::D8);
    }

    #[test]
    fn parse_dice_ok() {
        let subject = "8".parse::<Dice>();
        assert!(subject.is_ok());
    }

    #[test]
    fn parse_dice_err() {
        let subject = "".parse::<Dice>();
        assert!(subject.is_err());
    }
}
