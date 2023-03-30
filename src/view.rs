use crate::game_mod::Point;
use crate::Game;

pub fn draw(game: &Game) {
    // header units
    println!("  0 1 2 3");
    for y in 0..4 {
        for x in 0..4 {
            // spacing
            if x == 0 {
                print!("{}", y);
            }
            let point = Point { x, y };

            print!(" {}", '.');
        }
        // spacing
        println!();
    }
}
