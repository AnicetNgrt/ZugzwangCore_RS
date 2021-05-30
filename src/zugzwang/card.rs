use super::{Id, Action};

pub struct Card<'a> {
    pub id: Id,
    pub archetype: &'a CardArchetype
}

pub struct CardArchetype {
    pub name: String,
    pub max_use_turn: Option<u8>,
    pub max_use_game: Option<u8>,
    pub weight: u8,
    pub actions: CardAction
}

pub struct CardAction {
    pub name: String,
    pub cost: u8,
    pub action_generator: fn() -> Box<dyn Action>
}