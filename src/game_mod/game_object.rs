use super::point::Point;
use crate::Character;
use crate::Monster;

pub type GameObjectId = usize;

pub struct GameObjectCharacter {
    character: Character,
    position: Point,
    hp: i32,
    id: GameObjectId,
}

pub struct GameObjectMonster {
    monster: Monster,
    position: Point,
    hp: i32,
    id: GameObjectId,
}

pub enum GameObject {
    Character(GameObjectCharacter),
    Monster(GameObjectMonster),
}

pub trait IntoGameObject {
    fn into_game_object(self, id: GameObjectId) -> GameObject;
}

impl IntoGameObject for Character {
    fn into_game_object(self, id: GameObjectId) -> GameObject {
        GameObject::Character(GameObjectCharacter {
            hp: self.hp,
            character: self,
            position: Point::new(1, 2),
            id,
        })
    }
}

impl IntoGameObject for Monster {
    fn into_game_object(self, id: GameObjectId) -> GameObject {
        GameObject::Monster(GameObjectMonster {
            hp: self.hp,
            monster: self,
            position: Point::new(3, 2),
            id,
        })
    }
}

impl GameObject {
    pub fn get_ac(&self) -> i32 {
        match self {
            GameObject::Character(c) => 10,
            GameObject::Monster(m) => 10,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            GameObject::Character(c) => &c.character.name,
            GameObject::Monster(m) => &m.monster.name,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        match self {
            GameObject::Character(c) => c.hp -= damage,
            GameObject::Monster(m) => m.hp -= damage,
        }
    }

    pub fn get_hp(&self) -> i32 {
        match self {
            GameObject::Character(c) => c.hp,
            GameObject::Monster(m) => m.hp,
        }
    }

    pub fn set_position(&mut self, position: Point) {
        match self {
            GameObject::Character(c) => c.position = position,
            GameObject::Monster(m) => m.position = position,
        }
    }

    pub fn get_position(&self) -> Point {
        match self {
            GameObject::Character(c) => c.position,
            GameObject::Monster(m) => m.position,
        }
    }

    pub fn displace(&mut self, vector: Point) {
        self.set_position(self.get_position() + vector);
    }

    pub fn get_id(&self) -> GameObjectId {
        match self {
            GameObject::Character(c) => c.id,
            GameObject::Monster(m) => m.id,
        }
    }
}
