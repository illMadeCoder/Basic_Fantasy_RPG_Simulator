enum Species {
    Dwarf,
    Elf,       
    Halfling,
    Human
}

struct Character {
    name: String,
    species: Species,
    hp: i32,
    str: i32,
    dex: i32,
    int: i32,
    cha: i32,    
}

impl Character {
    pub fn new(name: &str,
	       species: Species,
	       hp: i32,
	       str: i32,
	       dex: i32,
	       int: i32,
	       cha: i32) -> Character {
	Character {
	    name: String::from(name),
	    species,
	    hp,
	    str,
	    dex,
	    int,
	    cha,
	}
    }
}
