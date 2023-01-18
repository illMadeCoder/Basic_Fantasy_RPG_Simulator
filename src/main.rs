use std::str::FromStr;
use rand::Rng;

#[derive(PartialEq)]
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

pub struct DiceColl(Vec<Dice>);

impl DiceColl {
    pub fn from(dice_coll : Vec<Dice>) -> DiceColl {
	DiceColl(dice_coll)
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

impl FromStr for DiceColl {
    type Err = ParseDiceError;
    fn from_str(s: &str) -> Result<DiceColl, Self::Err> {
	// TODO: cleanup impl
	
	// expects form <Number>D<Number>	
	let coll = s.split("d").collect::<Vec<&str>>();
	if coll.len() != 2 {
	    Err(ParseDiceError)
	} else {
	    let quantity_result = coll[0].parse::<u32>();
	    match quantity_result {
		Ok(quantity) => {
		    let mut dice_vec = Vec::new();
		    for _ in 0..quantity {
			let dice = coll[1].parse::<Dice>()?;
			dice_vec.push(dice);
		    }
		    Ok(DiceColl::from(dice_vec))
		},
		Err(_) => Err(ParseDiceError)
	    }
	}
    }
}

fn main() {
    loop {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).expect("Failed reading from stdin");
	let dice_coll_parse = input.trim().parse::<DiceColl>();
	match dice_coll_parse {
	    Ok(dice_coll) => println!("{:?}", dice_coll.roll_and_sum()),
	    Err(_) => println!("Poorly formed dice roll input. Try again with the form: XdY where X and Y are unsigned whole numbers")
	}
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
    fn parse_dice_coll_ok() {
	let dice_coll = "3d8".parse::<DiceColl>();
	assert!(dice_coll.is_ok());
    }

    #[test]
    fn parse_dice_coll_err() {
	let dice_coll = "8".parse::<DiceColl>();
	assert!(dice_coll.is_err());
    }

    #[test]
    fn parse_dice_coll_roll_and_sum() {
	let dice_coll = "3d8".parse::<DiceColl>().unwrap();
	let subject = dice_coll.roll_and_sum();
	assert!(subject == 24);
    }
}
