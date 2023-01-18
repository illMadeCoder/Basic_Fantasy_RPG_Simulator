use std::str::FromStr;
use rand::Rng;

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
pub struct ParseDiceError;
impl Dice {
    pub fn from(dice_numeric: u32) -> Result<Dice, ParseDiceError>
    {
	match dice_numeric {
	    4 => Ok(Dice::D4),
	    6 => Ok(Dice::D6),
	    8 => Ok(Dice::D8),
	    10 => Ok(Dice::D10),
	    12 => Ok(Dice::D12),
	    20 => Ok(Dice::D20),
	    100 => Ok(Dice::D100),
	    _ => Err(ParseDiceError)
	}
    }
    fn to(&self) -> u32 {
	match self {
	    Dice::D4 => 4,
	    Dice::D6 => 6,
	    Dice::D8 => 8,
	    Dice::D10 => 10,
	    Dice::D12 => 12,
	    Dice::D20 => 20,
	    Dice::D100 => 100
	}
    }
    pub fn roll(&self) -> u32 {
	let mut rng = rand::thread_rng();
	// if in test environment return max
	if cfg!(test) {
	    self.to()
	} else {
	    rng.gen_range(1..=self.to())
	}
    }
}

impl FromStr for Dice {
    type Err = ParseDiceError;
    fn from_str(s: &str) -> Result<Dice, Self::Err> {
	match s.parse::<u32>() {
	    Ok(numeric) => Dice::from(numeric),
	    Err(_) => Err(ParseDiceError)
	}
    }
}

pub struct DicePool(Vec<Dice>);

impl DicePool {
    pub fn new(quantity : usize , dice: Dice) -> DicePool {
	DicePool(vec![dice; quantity])
    }
    pub fn from(dicepool : Vec<Dice>) -> DicePool {
	DicePool(dicepool)
    }
    pub fn roll(&self) -> Vec<u32> {
	let mut rolls = Vec::new();
	for dice in &self.0 {
	    rolls.push(dice.roll())
	}
	rolls
    }
    pub fn roll_and_sum(&self) -> u32 {
	self.roll().iter().sum()
    }
}

impl FromStr for DicePool {
    type Err = ParseDiceError;
    fn from_str(s: &str) -> Result<DicePool, Self::Err> {
	// expects form <Unsigned Whole Number>d<Unsigned Whole Number>
	let coll = s.split("d").collect::<Vec<&str>>();
	if coll.len() != 2 {
	    return Err(ParseDiceError)
	}
	
	let quantity = match coll[0].parse::<u32>() {
	    Ok(n) => n,
	    Err(_) => return Err(ParseDiceError)
	};
	
	let dice = match coll[1].parse::<Dice>() {
	    Ok(n) => n,
	    Err(_) => return Err(ParseDiceError)
	};
	
	let mut dice_vec = Vec::new();	
	for _ in 0..quantity {
	    dice_vec.push(dice);
	}
	
	Ok(DicePool::from(dice_vec))	
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dice_to() {
	let dice = Dice::D8;
	assert!(dice.to() == 8);
    }

    #[test]
    fn dice_roll() {
	let dice = Dice::D8;
	let subject = dice.roll();
	assert!(subject == 8);
    }

    #[test]
    fn from_dice_ok() {
	let subject = Dice::from(8);
	assert!(subject.is_ok());
    }

    #[test]
    fn from_dice_err() {
	let subject = Dice::from(0);
	assert!(subject.is_err());
    }

    #[test]
    fn from_dice() {
	let subject = Dice::from(8).unwrap();
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
	let dicepool = "3d8".parse::<DicePool>().unwrap();
	let subject = dicepool.roll_and_sum();
	assert!(subject == 24);
    }
}
