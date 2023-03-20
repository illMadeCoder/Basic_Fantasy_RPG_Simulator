use crate::point::Point;
use crate::Game;

pub fn draw(game: &Game) {
    println!("  0 1 2 3");
    for y in 0..4 {
        for x in 0..4 {
            if x == 0 {
                print!("{}", y);
            }
            let c = match game.grid.get(&Point { x, y }) {
                None => '.',
                Some(g) => g.upgrade().unwrap().borrow().get_c(),
            };

            print!(" {}", c);
        }
        println!();
    }
}
