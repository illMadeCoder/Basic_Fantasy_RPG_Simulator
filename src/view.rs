use crate::point::Point;
use crate::Game;

pub fn draw(game: &Game) {
    for y in 0..10 {
        for x in 0..10 {
            let point = Point { x, y };
            // if game.character.position == point {
            //     print!("@")
            // } else if game.monster.position == point {
            //     print!("m");
            // } else {
            //     print!(".");
            // }
            print!(".");
        }
        print!("\n");
    }
}
