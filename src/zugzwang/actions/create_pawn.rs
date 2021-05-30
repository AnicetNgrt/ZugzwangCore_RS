use super::super::{Id, Game, Action, RulesError, Pawn, PawnState};

pub struct CreatePawn {
    pub pawn_id: Option<Id>
}

impl CreatePawn {
    pub fn new() -> Self {
        CreatePawn {
            pawn_id: None
        }
    }
}

impl Action for CreatePawn {
    fn play(&mut self, game: &mut Game) -> Result<(), RulesError> {
        let id = game.gen_pawn_id();
        game.pawns.insert(id, Pawn::new(id, PawnState::Unplaced));
        self.pawn_id = Some(id);
        Ok(())
    }

    fn unplay(&mut self, game: &mut Game) {
        if let Some(_) = self.pawn_id {
            game.pawns.retain(|&pawn_id, _| pawn_id != pawn_id);
        }
    }
}