use crate::dice::{Dice, DiceNumType, DiceError};
use std::str::FromStr;
use std::convert::From;

#[derive(Debug)]
pub enum DicePoolError {
    DiceError(DiceError),
    DicePoolParseError(String),    
    ParseIntError(std::num::ParseIntError)
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

pub struct DicePool(Vec<Dice>);

impl From<Vec<Dice>> for DicePool {
    fn from(dicepool : Vec<Dice>) -> Self {
	DicePool(dicepool)
    }
}

impl TryFrom<Vec<DiceNumType>> for DicePool {
    type Error = DicePoolError;
    
    fn try_from(dicepool : Vec<DiceNumType>) -> Result<Self, Self::Error> {
	let r : Result<Vec<Dice>, DiceError> = dicepool.into_iter().map(|x| Dice::try_from(x)).collect();
	// TODO: remove intermediate step for v, if we ? at the end of collect() it requires type anno
	let v = r?;
	Ok(DicePool(v))
    }
}

impl DicePool {
    pub fn new(quantity : usize , dice: Dice) -> DicePool {
	DicePool(vec![dice; quantity])
    }
    
    pub fn roll(&self) -> Vec<DiceNumType> {
	self.0.iter().map(|&x| x.roll()).collect()
    }
    
    pub fn roll_and_sum(&self) -> u32 {
	self.roll().iter().map(|&x| x as u32).sum()
    }
}

impl FromStr for DicePool {
    type Err = DicePoolError;
    
    fn from_str(s: &str) -> Result<DicePool, Self::Err> {
	let (quantity_str, dice_str) = s.split_once("d").ok_or(DicePoolError::DicePoolParseError(String::from(s)))?;
	let quantity = quantity_str.parse::<usize>()?;
	let dice = dice_str.parse::<Dice>()?;
	
	Ok(DicePool::from(vec![dice; quantity]))
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
	let dicepool = DicePool::new(3, Dice::D8);
	let subject = dicepool.roll_and_sum();
	assert!(subject == 24);
    }

    #[test]
    fn parse_dicepool_parse_int_error() {
	assert!("ad8".parse::<DicePool>().is_err())
    }

    // #[test]
    // fn vec_dicepool_from_dicenumtype_ok() {
    // 	let subject = DicePool::try_from(vec!([1, 2, 3]));
    // }
}


