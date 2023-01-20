#[derive(Debug)]
pub enum Species {
    Dwarf,
    Elf,       
    Halfling,
    Human
}

#[derive(Debug)]
pub enum Class {
    Cleric,
    Fighter,       
    MagicUser,
    Thief    
}

pub type SkillType = i64;
pub type HpType = i64;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub species: Species,
    pub class: Class,
    pub hp: HpType,
    pub str: SkillType,
    pub dex: SkillType,
    pub int: SkillType,
    pub wis: SkillType,
    pub cha: SkillType,    
}
