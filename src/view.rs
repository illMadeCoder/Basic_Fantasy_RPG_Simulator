use crate::point::Point;
use crate::Game;

pub fn draw(game: &Game) {
    for y in 0..4 {
        for x in 0..4 {
            let c = match game.grid.get(Point {
                x: x as i32,
                y: y as i32,
            }) {
                None => '.',
                Some(g) => g.borrow().get_c(),
            };

            print!(" {}", c);
        }
        print!("\n");
    }
}
