use super::{Pacman, Id};

pub type Pawns = Vec<Pawn>;

#[derive(Copy, Clone, Debug)]
pub enum PawnState {
    Placed(Pacman),
    Unplaced
}

#[derive(Copy, Clone)]
pub struct Pawn {
    pub id: Id,
    pub state: PawnState
}

impl Pawn {
    pub fn new(id: Id, state: PawnState) -> Self {
        Pawn { id, state }
    }
}