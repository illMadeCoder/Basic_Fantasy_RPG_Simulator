use rand::Rng;

fn roll(dice : u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=dice)	
}

// #[derive(Debug)]
// struct Creature {
//     hp: i32
// }

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("error");
    match input.trim() {
	"roll" => println!("{}", roll(20)),
	_ => println!("b")
    }
}
