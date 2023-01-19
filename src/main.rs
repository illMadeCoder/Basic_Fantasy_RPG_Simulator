mod dice;
mod character;
mod dicepool;

use dice::*;
use dicepool::{DicePool, DicePoolError, DiceRollSum};
use character::*;

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    //write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    String::from(input.trim())
}

// TODO: Hacky code, needs total rewrite
fn roll_dicepool_prompt(opt_mandatory_roll: Option<&str>) -> DiceRollSum {  
    loop {
	let mut input = get_input(); 
	if let Some(mandatory_roll) = opt_mandatory_roll  {
	    while input != String::from(mandatory_roll) {
		println!("Expected input {}", mandatory_roll);
		input = get_input();
	    }
	} 

	let dicepool_parse = input.trim().parse::<DicePool>();
	match dicepool_parse {
	    Ok(dicepool) => return dicepool.roll_and_sum(),
	    Err(_) => println!("Poorly formed dice roll input. Try again with the form: XdY where X and Y are unsigned whole numbers")
	}
    }
}

fn prompt_character_stat_roll(prompt_stat_name : &str) -> SkillType {
    println!("Roll 3d6 for {}:", prompt_stat_name);
    let dice_roll_sum = roll_dicepool_prompt(Some("3d6"));
    println!("{:?}", dice_roll_sum);
    dice_roll_sum.2 as SkillType
}

fn main() {
    loop {
	// let mut input = String::new();
	// std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
	// println!("creating character");
	// pause();
	// println!("1. rolling stats");
	// pause();
	// println!("rolling strength");
	// println!("roll 3d6");
	
	// TODO: potentially use reflection or something on a Skill type to prompt for each stat
	let str_roll = prompt_character_stat_roll("Strength");
	println!();
	let dex_roll = prompt_character_stat_roll("Dexterity");
	println!();
	let int_roll = prompt_character_stat_roll("Intelligence");
	println!();
	let wis_roll = prompt_character_stat_roll("Wisdom");
	println!();
	let cha_roll = prompt_character_stat_roll("Charisma");
	println!();
	//pause();

	// let stats_dicepool = DicePool::new(3,Dice::D6);
	// let roll = stats_dicepool.roll();
	// println!("{:?} = {sum}", roll);
	// let str = sum;
	
	// println!("rolling dexterity");
	// println!("3d6");
	// pause();
	// let stats_dicepool = DicePool::new(3,Dice::D6);
	// let roll = stats_dicepool.roll();
	// let sum : DiceNumType = roll.iter().sum();
	// println!("{:?} = {sum}", roll);
	// let dex = sum;
	
    }
}
