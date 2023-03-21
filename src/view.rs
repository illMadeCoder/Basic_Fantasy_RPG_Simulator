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
            let c = match game.get_at(&point) {
                Some(game_object) => match game.get_ref(game_object) {
                    crate::game_mod::GameObject::Character { .. } => 'C',
                    crate::game_mod::GameObject::Monster { .. } => 'M',
                },
                None => '.',
            };

            print!(" {}", c);
        }
        // spacing
        println!();
    }
}
