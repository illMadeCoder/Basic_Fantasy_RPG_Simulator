pub struct View {}

impl View {
    pub fn draw(game: &Game) {
        for y in 0..game.height {
            for x in 0..game.width {
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
}
