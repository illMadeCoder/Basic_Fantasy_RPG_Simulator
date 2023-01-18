use crate::dice::*;
use std::str::FromStr;

pub struct DicePool(Vec<Dice>);

impl DicePool {
    pub fn new(quantity : usize , dice: Dice) -> DicePool {
	DicePool(vec![dice; quantity])
    }
    pub fn from(dicepool : Vec<Dice>) -> DicePool {
	DicePool(dicepool)
    }
    pub fn roll(&self) -> Vec<DiceN> {
	let mut rolls = Vec::new();
	for dice in &self.0 {
	    rolls.push(dice.roll())
	}
	rolls
    }
    pub fn roll_and_sum(&self) -> DiceN {
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
	
	let quantity = match coll[0].parse::<DiceN>() {
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
	let dicepool = "3d8".parse::<DicePool>().unwrap();
	let subject = dicepool.roll_and_sum();
	assert!(subject == 24);
    }
}
