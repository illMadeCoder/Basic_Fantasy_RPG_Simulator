#![allow(dead_code)]

mod character;
mod dice_expr;
mod game;
mod item;
mod monster;
mod point;

use character::Character;
use dice_expr::{DicePool, DiceRollSum};
use game::{Actor, Agent, Potential};
use game::{Game, GameAction, GameObject};
use monster::Monster;

use std::collections::HashSet;
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

// pub struct GameObject {
//     String name,
//     usize id,
// }

// enum GameObjectComponent {
//     Attackable {  },
//     Moveable { }
// }

// pub struct Game {
//     pub game_objects: Vec<&dyn GameObject>,
//     pub width: i32,
//     pub height: i32,
//     pub turn: i32,
// }

// impl Game {
//     fn update(&mut self) {

//         // for agent in agents
//         // agent.get

//         // println!(
//         //     "\nturn {2} c: {0} e: {1}",
//         //     self.character.hp, self.monster.hp, self.turn
//         // );
//         // if self.turn % 2 == 0 {
//         //     self.character.take_turn(self);
//         // } else {
//         //     self.monster.take_turn(self);
//         // }

//         // self.turn += 1;

//         // let winner: &str = if self.character.hp > 0 {
//         //     &self.character.name
//         // } else {
//         //     &self.monster.name
//         // };
//         // println!("\n{0} won!", winner);
//     }
// }

pub struct PrototypeAgent {
    actor: usize,
}

impl Agent for PrototypeAgent {
    fn decide_action(&self, game: &Game) -> GameAction {
        let character = game.get(self.actor);
        let potential_set = character.potential_set();
        if potential_set.contains(&Potential::Attack) {
            GameAction::MeleeAttack {
                target: *game.get_by_name("Goblin".to_string()).first().unwrap(),
            }
        } else {
            GameAction::None
        }
    }
}

impl Actor for Character {
    fn potential_set(&self) -> HashSet<Potential> {
        let mut set = HashSet::new();
        set.insert(Potential::Attack);
        set
    }
}

impl Actor for Monster {
    fn potential_set(&self) -> HashSet<Potential> {
        let set = HashSet::new();
        //set.insert(Potential::Attack);
        set
    }
}

// pub trait HasAC {
//     fn ac(&self) -> i32;
// }

// pub trait HasHP {
//     fn get_max_hp(&self) -> i32;
//     fn get_hp(&self) -> i32;
//     fn set_hp(&mut self, damage: i32);
// }

// pub trait Attackable {
//     fn is_hit(&self, hit_roll: i32) -> bool;
//     fn apply_damage(&mut self, damage: i32);
//     fn attack(&mut self, hit_roll: i32, damage: i32) -> bool {
//         if self.is_hit(hit_roll) {
//             self.apply_damage(damage);
//             return true;
//         }
//         false
//     }
// }

// impl<T: HasAC + HasHP + GameObject> Attackable for T {
//     fn is_hit(&self, hit_roll: i32) -> bool {
//         hit_roll >= self.ac()
//     }

//     fn apply_damage(&mut self, damage: i32) {
//         self.set_hp(self.get_hp() - damage)
//     }

//     fn attack(&mut self, hit_roll: i32, damage: i32) -> bool {
//         if self.is_hit(hit_roll) {
//             self.apply_damage(damage);
//             true
//         } else {
//             false
//         }
//     }
// }

// pub trait AsPoint {
//     fn as_point(&mut self) -> &mut Point;
// }

// pub trait Moveable {
//     fn displace(&mut self, vector: Point);
// }

// impl<T: AsPoint> Moveable for T {
//     fn displace(&mut self, vector: Point) {
//         let p = self.as_point();
//         *p = *p + vector;
//     }
// }

// pub enum GameActionType<'a> {
//     MeleeAttack {
//         attack: DicePool,
//         damage: DicePool,
//         target: &'a dyn Attackable,
//     },
//     MissileAttack,
//     Move {
//         vector: Point,
//         target: &'a dyn Moveable,
//     },
// }

impl GameObject for Monster {
    fn get_ac(&self) -> i32 {
        10
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
    fn get_hp(&self) -> i32 {
        self.hp
    }
}

impl GameObject for Character {
    fn get_ac(&self) -> i32 {
        10
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
    fn get_hp(&self) -> i32 {
        self.hp
    }
}

fn main() {
    let mut character = Character::gen();
    let mut monster = Monster::gen();
    let mut game = Game::new();
    let character_id = game.insert(&mut character);
    let _monster_id = game.insert(&mut monster);

    let agent_character = PrototypeAgent {
        actor: character_id,
    };
    let action_character = agent_character.decide_action(&game);
    game.apply(action_character);
}
