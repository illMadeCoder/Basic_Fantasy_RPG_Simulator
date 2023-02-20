#![allow(dead_code)]

mod ability;
mod ability_score;
mod ability_score_set;
mod action;
mod agent;
mod ancestry;
mod character;
mod character_error;
mod class;
mod dice;
mod dicepool;
mod item;
mod monster;
mod point;

use action::{Action, ActionType, Attackable, HasAC, HasHP, HasName};
use agent::Agent;
use character::Character;
use dice::Dice;
use dicepool::{DicePool, DiceRollSum};
use monster::Monster;
use point::Point;

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
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    String::from(input.trim())
}

// TODO: Hacky code, needs total rewrite
fn roll_dicepool_prompt(opt_mandatory_roll: Option<&str>) -> DiceRollSum {
    loop {
        let mut input = get_input();
        if let Some(mandatory_roll) = opt_mandatory_roll {
            while input != String::from(mandatory_roll) {
                println!("Expected input {}", mandatory_roll);
                input = get_input();
            }
        }

        let dicepool_parse = input.trim().parse::<DicePool>();
        match dicepool_parse {
	    Ok(dicepool) => return dicepool.dice_roll_sum(),
	    Err(_) => println!("Poorly formed dice roll input. Try again with the form: XdY where X and Y are unsigned whole numbers")
	}
    }
}

fn prompt_character_stat_roll(prompt_stat_name: &str) -> i32 {
    println!("Roll 3d6 for {}:", prompt_stat_name);
    let dice_roll_sum = roll_dicepool_prompt(Some("3d6"));
    println!("{:?}", dice_roll_sum);
    dice_roll_sum.sum as i32
}

// fn prompt_create_character() -> Character {
//     // TODO: potentially use reflection or something on a Skill type to prompt for each stat
//     let str_roll = prompt_character_stat_roll("Strength");
//     println!();
//     let dex_roll = prompt_character_stat_roll("Dexterity");
//     println!();
//     let int_roll = prompt_character_stat_roll("Intelligence");
//     println!();
//     let wis_roll = prompt_character_stat_roll("Wisdom");
//     println!();
//     let cha_roll = prompt_character_stat_roll("Charisma");
//     println!();
//     let con_roll = prompt_character_stat_roll("Charisma");
//     println!();
//     // choose a race
//     // minimum and maximum skills for races
//     // race powers
//     // choose a class
//     // your class must meet the prime requisite for you class
//     // magic user spells
//     // 0 xp
//     // xp to advance per class
//     // roll hit dice for class adding con
//     // roll starting money 3d6 * 10 gold
//     // purchase equipment
//     // determine ac
//     // determine attack bonus
//     // saving throws
//     // name character
//     // Character {
//     // 	name: String::from("hello"),
//     // 	species: Species::Human,
//     // 	class: Class::Fighter,
//     // 	ability_score_set: AbilityScoreSet {str: str_roll,
//     // 				       dex: dex_roll,
//     // 				       int: int_roll,
//     // 				       wis: wis_roll,
//     // 				       cha: cha_roll,
//     // 				       con: con_roll}
//     // }
// }

// fn prompt_character_stat_roll(prompt_stat_name : &str) -> AbilityScoreType {
// }

pub struct Game {
    pub character: Character,
    pub monster: Monster,
    pub width: i32,
    pub height: i32,
}

impl Game {
    fn update(&mut self) {
        let mut turn = 0;
        while self.character.hp > 0 && self.monster.hp > 0 {
            println!(
                "\nturn {2} c: {0} e: {1}",
                self.character.hp, self.monster.hp, turn
            );
            if turn % 2 == 0 {
                self.character.take_turn(&mut self.monster);
            } else {
                self.monster.take_turn(&mut self.character);
            }
            turn += 1;
        }

        let winner: &str = if self.character.hp > 0 {
            &self.character.name
        } else {
            &self.monster.name
        };
        println!("\n{0} won!", winner);
    }
}

pub struct View {}

impl View {
    pub fn draw(game: &Game) {
        for y in 0..game.height {
            for x in 0..game.width {
                let point = Point { x, y };
                if game.character.position == point {
                    print!("@")
                } else if game.monster.position == point {
                    print!("m");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

fn main() {
    let mut character = Character::gen();
    println!("{:#?}", character);

    let mut monster = Monster {
        name: "Goblin".to_string(),
        ac: 14,
        hit_dice: 1,
        no_of_attacks: 1,
        damage: DicePool::new(1, Dice::D6),
        hp: 8,
        max_hp: 8,
        position: Point { x: 4, y: 3 },
    };

    let mut game = Game {
        character,
        monster,
        width: 10,
        height: 5,
    };

    game.update();
    View::draw(&game);
}
