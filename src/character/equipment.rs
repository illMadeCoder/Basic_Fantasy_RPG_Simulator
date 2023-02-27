use crate::item::Item;

#[derive(Debug)]
pub enum HandEquipType {
    OneHanded(Item),
    DualHanded(Item, Item),
    TwoHanded(Item),
    None,
}

#[derive(Debug)]
pub struct Equipment {
    pub hand: HandEquipType,
    pub armor: Option<Item>,
}
