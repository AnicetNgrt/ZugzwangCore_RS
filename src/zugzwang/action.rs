use super::{Game};

#[derive(Debug)]
pub enum RulesError {
    PositionTaken,
    IllegalStateTransition,
    PawnNotExists,
    MaximumPawnCount
}

pub trait Action {
    fn play(&mut self, game: &mut Game) -> Result<(), RulesError>;
    fn unplay(&mut self, game: &mut Game);
}