use super::super::{Id, Game, Action, RulesError, Pawn, PawnState};

pub struct CreatePawn {
    pub pawn_id: Id
}

impl CreatePawn {
    pub fn new() -> Self {
        CreatePawn {
            pawn_id: 0
        }
    }
}

impl Action for CreatePawn {
    fn play(&mut self, game: &mut Game) -> Result<(), RulesError> {
        let id = game.gen_pawn_id();
        game.pawns.insert(id, Pawn::new(id, PawnState::Unplaced));
        self.pawn_id = id;
        Ok(())
    }

    fn unplay(&mut self, game: &mut Game) -> Result<(), RulesError> {
        game.pawns.retain(|&pawn_id, _| pawn_id != self.pawn_id);
        Ok(())
    }
}