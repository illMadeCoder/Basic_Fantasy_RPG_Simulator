use crate::dicepool::DicePool;

//use crate::dice::Dice;

trait IntoItem {
    fn name(&self) -> &str;
    fn weight(&self) -> &f32;
    fn price(&self) -> &i32;
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub weight: f32,
    pub price: i32,
}

#[derive(Debug)]
pub enum WeaponSize {
    S,
    M,
    L,
}

#[derive(Debug)]
pub struct Weapon {
    pub item: Item,
    pub size: WeaponSize,
    pub damage: DicePool,
}

// pub const SHORTSWORD : Weapon = Weapon {
//     item: Item {
// 	name: String::from("Sword"),
// 	weight: 3.0,
// 	price: 6
//     },
//     size: WeaponSize::S,
//     damage: DicePool::new(1, Dice::D6)
// };
