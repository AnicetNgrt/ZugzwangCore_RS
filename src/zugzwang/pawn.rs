use super::{Pacman, Id};

use std::collections::HashMap;

pub type Pawns = HashMap<Id, Pawn>;

#[derive(Copy, Clone, Debug)]
pub enum PawnState {
    Placed(Pacman),
    Unplaced
}

#[derive(Copy, Clone, Debug)]
pub struct Pawn {
    pub id: Id,
    pub state: PawnState,
    pub owner_id: Option<Id>
}

impl Pawn {
    pub fn new(id: Id, state: PawnState) -> Self {
        Pawn { id, state, owner_id: None }
    }
}