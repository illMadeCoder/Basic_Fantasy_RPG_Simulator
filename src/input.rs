// use std::io;
// use std::io::prelude::*;

// use crate::game_mod::{Direction};

// // fn pause() {
// //     let mut stdin = io::stdin();
// //     let mut stdout = io::stdout();

// //     // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
// //     //write!(stdout, "Press any key to continue...").unwrap();
// //     stdout.flush().unwrap();

// //     // Read a single byte and discard
// //     let _ = stdin.read(&mut [0u8]).unwrap();
// // }

// // fn get_input() -> String {
// //     let mut input = String::new();
// //     std::io::stdin()
// //         .read_line(&mut input)
// //         .expect("Failed to read from stdin");
// //     String::from(input.trim())
// // }

// // // TODO: Hacky code, needs total rewrite
// // fn roll_dicepool_prompt(opt_mandatory_roll: Option<&str>) -> DiceRollSum {
// //     loop {
// //         let mut input = get_input();
// //         if let Some(mandatory_roll) = opt_mandatory_roll {
// //             while input != String::from(mandatory_roll) {
// //                 println!("Expected input {}", mandatory_roll);
// //                 input = get_input();
// //             }
// //         }

// //         let dicepool_parse = input.trim().parse::<DicePool>();
// //         match dicepool_parse {
// // 	    Ok(dicepool) => return dicepool.dice_roll_sum(),
// // 	    Err(_) => println!("Poorly formed dice roll input. Try again with the form: XdY where X and Y are unsigned whole numbers")
// // 	}
// //     }
// // }

// // fn prompt_character_stat_roll(prompt_stat_name: &str) -> i32 {
// //     println!("Roll 3d6 for {}:", prompt_stat_name);
// //     let dice_roll_sum = roll_dicepool_prompt(Some("3d6"));
// //     println!("{:?}", dice_roll_sum);
// //     dice_roll_sum.sum as i32
// // }

// pub fn game_action() -> GameAction {
//     let stdin = std::io::stdin();
//     let mut buf = String::new();
//     stdin.read_line(&mut buf).unwrap();
//     let trimmed = buf.trim().to_string();
//     let mut split = trimmed.split(' ');
//     let action = split.next().unwrap();
//     if action == "move" {
//         let dir_str = split.next().unwrap();
//         let direction: Direction = dir_str.parse().unwrap();
//         GameAction::Move {
//             direction,
//             body: GameBody::Character(0),
//         }
//     } else if action == "attack" {
//         GameAction::MeleeAttack {
//             source: GameAttackSource::Character(0),
//             target: GameAttackTarget::Monster(0),
//         }
//     } else {
//         GameAction::None
//     }
// }
