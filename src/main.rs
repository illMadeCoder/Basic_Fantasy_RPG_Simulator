mod dice;
mod character;
use dice::*;
use character::*;

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn roll_dice_prompt() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    let dicepool_parse = input.trim().parse::<DicePool>();
    match dicepool_parse {
	Ok(dicepool) => println!("{:?}", dicepool.roll_and_sum()),
	Err(_) => println!("Poorly formed dice roll input. Try again with the form: XdY where X and Y are unsigned whole numbers")
    }    
}

fn main() {
    loop {
	// let mut input = String::new();
	// std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
	println!("creating character");
	pause();
	println!("1. rolling stats");
	pause();
	println!("rolling strength");
	println!("3d6");
	pause();
	let stats_dicepool = DicePool::new(3,Dice::D6);
	let roll = stats_dicepool.roll();
	let sum : u32 = roll.iter().sum();	
	println!("{:?} = {sum}", roll);
	let str = sum;
	
	println!("rolling dexterity");
	println!("3d6");
	pause();
	let stats_dicepool = DicePool::new(3,Dice::D6);
	let roll = stats_dicepool.roll();
	let sum : u32 = roll.iter().sum();
	println!("{:?} = {sum}", roll);
	let dex = sum;
    }
}
