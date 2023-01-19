pub enum Species {
    Dwarf,
    Elf,       
    Halfling,
    Human
}

pub type SkillType = i64;
pub type HpType = i64;

pub struct Character {
    name: String,
    species: Species,
    hp: HpType,
    str: SkillType,
    dex: SkillType,
    int: SkillType,
    wis: SkillType,
    cha: SkillType,    
}

impl Character {
    pub fn new(name: &str,
	       species: Species,
	       hp: HpType,
	       str: SkillType,
	       dex: SkillType,
	       int: SkillType,
	       wis: SkillType,
	       cha: SkillType) -> Character {
	Character {
	    name: String::from(name),
	    species,
	    hp,
	    str,
	    dex,
	    int,
	    wis,
	    cha,
	}
    }
}
