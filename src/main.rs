#![allow(dead_code)]

mod ability;
mod ability_score;
mod ability_score_set;
mod ancestry;
mod character;
mod character_error;
mod class;
mod dice;
mod dicepool;
mod item;

use character::Character;
use dice::Dice;
use dicepool::{DicePool, DiceRollSum};

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

struct Monster {
    name: String,
    ac: i32,
    hit_dice: u8,
    no_of_attacks: u8,
    damage: DicePool,
    hp: i32,
}

// Goblin
// Armor Class:	14 (11)
// Hit Dice:	1-1
// No. of Attacks:	1 weapon
// Damage:	1d6 or by weapon
// Movement:	20' Unarmored 30'
// No. Appearing:	2d4 ,Wild 6d10, Lair 6d10
// Save As:	Fighter: 1
// Morale:	7 or see below
// Treasure Type:	R each; C in lair
// XP:	10

fn main() {
    let mut character = Character::gen();
    println!("{:#?}", character);

    let mut m = Monster {
        name: "Goblin".to_string(),
        ac: 14,
        hit_dice: 1,
        no_of_attacks: 1,
        damage: DicePool::new(1, Dice::D6),
        hp: 8,
    };

    let mut turn = 0;
    while character.hp > 0 && m.hp > 0 {
        println!("\nturn {2} c: {0} e: {1}", character.hp, m.hp, turn);
        if turn % 2 == 0 {
            attack_monster(&character, &mut m);
        } else {
            attack_player(&m, &mut character);
        }
        turn += 1;
    }
}

fn attack_player(m: &Monster, c: &mut Character) {
    // attack monster
    let attack_dice_pool = DicePool::new(1, Dice::D20);
    println!("{0} attacks {1}", m.name, c.name);

    let roll = attack_dice_pool.dice_roll_sum().sum;
    println!("{0} rolls {1} to attack against ac {2}", m.name, roll, c.ac);

    let hit = roll >= m.ac;
    let hit_or_miss_text = if hit { "hit" } else { "missed" };
    println!("{0} {1} {2}", m.name, hit_or_miss_text, c.name);

    if hit {
        let damage_dice = DicePool::new(1, Dice::D6);
        let damage_roll = damage_dice.dice_roll_sum().sum;
        c.hp -= damage_roll;
        println!("{0} takes {1} damage", c.name, damage_roll)
    }
}

fn attack_monster(c: &Character, m: &mut Monster) {
    // attack monster
    let attack_dice_pool = DicePool::new(1, Dice::D20);
    println!("{0} attacks {1}", c.name, m.name);

    let roll = attack_dice_pool.dice_roll_sum().sum;
    println!("{0} rolls {1} to attack against ac {2}", c.name, roll, m.ac);

    let hit = roll >= m.ac;
    let hit_or_miss_text = if hit { "hit" } else { "missed" };
    println!("{0} {1} {2}", m.name, hit_or_miss_text, c.name);

    if hit {
        let damage_dice = DicePool::new(1, Dice::D6);
        let damage_roll = damage_dice.dice_roll_sum().sum;
        m.hp -= damage_roll;
        println!("{0} takes {1} damage", m.name, damage_roll)
    }
}
