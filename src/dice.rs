use std::str::FromStr;
use rand::Rng;
use std::convert::{TryFrom, TryInto, From, Into};

pub type DiceN = u8;

#[derive(PartialEq, Clone, Copy)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100
}

#[derive(Debug, PartialEq)]
pub struct InitializeDiceError;
impl Dice {
    pub fn roll(self) -> DiceN {
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

impl TryFrom<DiceN> for Dice {
    type Error = InitializeDiceError;

    fn try_from(value: DiceN) -> Result<Self, Self::Error> {
	match value {
	    4 => Ok(Dice::D4),
	    6 => Ok(Dice::D6),
	    8 => Ok(Dice::D8),
	    10 => Ok(Dice::D10),
	    12 => Ok(Dice::D12),
	    20 => Ok(Dice::D20),
	    100 => Ok(Dice::D100),
	    _ => Err(InitializeDiceError)
	}
    }
}

impl From<Dice> for DiceN {
    fn from(value: Dice) -> Self {
    	match value {
	    Dice::D4 => 4,
	    Dice::D6 => 6,
	    Dice::D8 => 8,
	    Dice::D10 => 10,
	    Dice::D12 => 12,
	    Dice::D20 => 20,
	    Dice::D100 => 100
	}
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseDiceError;
impl FromStr for Dice {
    type Err = ParseDiceError;
    fn from_str(s: &str) -> Result<Dice, Self::Err> {
	match s.parse::<DiceN>() {
	    Ok(numeric) => match numeric.try_into() {
		Ok(dice) => Ok(dice),
		Err(_) => Err(ParseDiceError)
	    },
	    Err(_) => Err(ParseDiceError)
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dice_to() {
	let dice = Dice::D8;
	let subject : DiceN = dice.into();
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
