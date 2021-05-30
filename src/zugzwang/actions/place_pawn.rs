use super::super::{Action, Id, RulesError, Game, PawnState, Pacman};

pub struct PlacePawn<'a> {
    pub pawn_id: Id,
    pub position: &'a Pacman
}

impl<'a> PlacePawn<'a> {
    pub fn new(pawn_id: Id, position: &'a Pacman) -> Self {
        PlacePawn { pawn_id, position }
    }
}

impl<'a> Action for PlacePawn<'a> {
    fn play(&mut self, game: &mut Game) -> Result<(), RulesError> {
        if !game.is_position_free(self.position) {
            return Err(RulesError::PositionTaken);
        }

        let pawn = game.pawns.get_mut(&self.pawn_id);
        let mut pawn = match pawn {
            Some(pawn) => pawn,
            None => return Err(RulesError::PawnNotExists)
        };

        match pawn.state {
            PawnState::Unplaced => {
                pawn.state = PawnState::Placed(*self.position);
                Ok(())
            },
            _ => Err(RulesError::IllegalStateTransition)
        }
    }

    fn unplay(&mut self, game: &mut Game) {
        let pawn = game.pawns.get_mut(&self.pawn_id);
        if let Some(pawn) = pawn {
            pawn.state = PawnState::Unplaced;
        }
    }
}