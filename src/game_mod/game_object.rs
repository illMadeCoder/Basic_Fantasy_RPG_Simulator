// use super::point::Point;
// use crate::Character;
// use crate::Monster;

// pub type GameObjectId = usize;

// pub enum GameObject {
//     Character {
//         character: Character,
//         position: Point,
//         hp: i32,
//         id: GameObjectId,
//         agent: Agent,
//     },
//     Monster {
//         monster: Monster,
//         position: Point,
//         hp: i32,
//         id: GameObjectId,
//         agent: Agent,
//     },
// }

// pub trait IntoGameObject {
//     fn into_game_object(self, id: GameObjectId) -> GameObject;
// }

// impl IntoGameObject for Character {
//     fn into_game_object(self, id: GameObjectId) -> GameObject {
//         GameObject::Character {
//             hp: self.hp,
//             character: self,
//             position: Point::new(1, 2),
//             id,
//             agent: Agent::PlayerAgent { actor: id },
//         }
//     }
// }

// impl IntoGameObject for Monster {
//     fn into_game_object(self, id: GameObjectId) -> GameObject {
//         GameObject::Monster {
//             hp: self.hp,
//             monster: self,
//             position: Point::new(3, 2),
//             id,
//             agent: Agent::PrototypeAgent { actor: id },
//         }
//     }
// }

// impl GameObject {
//     pub fn get_ac(&self) -> i32 {
//         match self {
//             GameObject::Character { .. } => 10,
//             GameObject::Monster { .. } => 10,
//         }
//     }

//     pub fn get_name(&self) -> &str {
//         match self {
//             GameObject::Character { character, .. } => &character.name,
//             GameObject::Monster { monster, .. } => &monster.name,
//         }
//     }

//     pub fn take_damage(&mut self, damage: i32) {
//         match self {
//             GameObject::Character { hp, .. } => *hp -= damage,
//             GameObject::Monster { hp, .. } => *hp -= damage,
//         }
//     }

//     pub fn get_hp(&self) -> i32 {
//         match self {
//             GameObject::Character { hp, .. } => *hp,
//             GameObject::Monster { hp, .. } => *hp,
//         }
//     }

//     pub fn set_position(&mut self, position: Point) {
//         match self {
//             GameObject::Character { position: p, .. } => *p = position,
//             GameObject::Monster { position: p, .. } => *p = position,
//         }
//     }

//     pub fn get_position(&self) -> Point {
//         match self {
//             GameObject::Character { position, .. } => *position,
//             GameObject::Monster { position, .. } => *position,
//         }
//     }

//     pub fn displace(&mut self, vector: Point) {
//         self.set_position(self.get_position() + vector);
//     }

//     pub fn get_id(&self) -> GameObjectId {
//         match self {
//             GameObject::Character { id, .. } => *id,
//             GameObject::Monster { id, .. } => *id,
//         }
//     }

//     pub fn next_action(&self) -> GameAction {
//         match self {
//             GameObject::Character { agent, .. } => agent.next_action(),
//             GameObject::Monster { agent, .. } => agent.next_action(),
//         }
//     }
// }
