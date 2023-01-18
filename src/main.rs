use std::str::FromStr;
use rand::Rng;

enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100
}

#[derive(Debug)]
struct ParseDiceError;
impl Dice {
    fn from(dice_type_numeric: u32) -> Result<Dice, ParseDiceError>
    {
	match dice_type_numeric {
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
    fn roll(&self) -> u32 {
	let mut rng = rand::thread_rng();
	rng.gen_range(1..=self.to())
    }    
}

impl FromStr for Dice {
    type Err = ParseDiceError;
    fn from_str(s: &str) -> Result<Dice, Self::Err> {
	Dice::from(s.parse::<u32>().unwrap())
    }
}

struct DiceColl(Vec<Dice>);

impl DiceColl {
    fn from(dice_coll : Vec<Dice>) -> DiceColl {
	DiceColl(dice_coll)
    }
    fn roll(&self) -> Vec<u32> {
	let mut rolls = Vec::new();
	for dice in &self.0 {
	    rolls.push(dice.roll())
	}
	rolls
    }
    fn roll_and_sum(&self) -> u32 {
	self.roll().iter().sum()
    }
}

impl FromStr for DiceColl {
    type Err = ParseDiceError;
    fn from_str(s: &str) -> Result<DiceColl, Self::Err> {
	// expects form <Number>D<Number>	
	let coll = s.split("d").collect::<Vec<&str>>();
	let quantity = coll[0].parse::<u32>().unwrap();
	let mut dice_vec = Vec::new();
	for _ in 0..quantity {
	    dice_vec.push(coll[1].parse::<Dice>().unwrap());
	}
	Ok(DiceColl::from(dice_vec))	 
    }
}

fn main() {
    loop {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).expect("error");
	let dice_coll = input.trim().parse::<DiceColl>().unwrap();
	println!("{:?}", dice_coll.roll_and_sum());   
    }
}

#[test]
fn roll_dice_test() {
    let dice = Dice::D8;
    let subject = dice.roll();
    assert!(subject > 0 && subject <= 8)
}

#[test]
fn from_dice_test() {
    assert!(Dice::from(8).unwrap().to() == 8)
}

#[test]
fn parse_dice_test() {
    assert!("8".parse::<Dice>().unwrap().to() == 8)
}

#[test]
fn parse_dice_coll_test() {
    let dice_coll = "3d8".parse::<DiceColl>().unwrap();
    let subject = dice_coll.roll_and_sum();
    assert!(subject > 0 && subject <= 24);
}
