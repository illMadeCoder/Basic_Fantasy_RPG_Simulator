use crate::dicepool::DicePool;
use crate::dice::Dice;

struct Item {
    name : &str,
    weight : f32,
    price : i32,    
}

enum WeaponSize {
    S,
    M,
    L
}

struct Weapon {
    item: Item,
    size: WeaponSize,
    damage: DicePool
}

const SHORTSWORD : Weapon = Weapon {
    item: Item {
	name: "Sword",
	weight: 3,
	price: 6
    },
    size: WeaponSize::S,
    damage: DicePool::new(1, Dice::D6)
};
