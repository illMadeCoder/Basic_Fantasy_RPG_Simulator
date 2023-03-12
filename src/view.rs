use crate::point::Point;
use crate::Game;

pub fn draw(game: &Game) {
    for y in 0..game.grid.height {
        for x in 0..game.grid.width {
            let c = match game.grid.get_at(Point { x, y }) {
                None => '.',
                Some(g) => g.borrow().get_c(),
            };
            print!(" {}", c);
        }
        print!("\n");
    }
}
