mod dice;

use dice::*;

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
