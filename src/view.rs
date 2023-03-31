use crate::game_mod::{Game, Point};

pub fn draw(game: &Game) {
    // header units
    println!("  0 1 2 3 4 5 6 7");
    for y in 0..8 {
        for x in 0..8 {
            // spacing
            if x == 0 {
                print!("{}", y);
            }
            let point = Point { x, y };

            let c = game
                .actor_at(point)
                .map(|a| a.name.chars().next().unwrap()) // first char of name
                .unwrap_or('.');

            print!(" {}", c);
        }
        // spacing
        println!();
    }
}
