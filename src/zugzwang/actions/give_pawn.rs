use super::super::{Action, Id, RulesError, Game};

pub struct GivePawn {
    pub player_id: Id,
    pub pawn_id: Id,
    previous_owner: Option<Id>
}

impl GivePawn {
    pub fn new(player_id: Id, pawn_id: Id) -> Self {
        GivePawn {
            player_id, pawn_id,
            previous_owner: None
        }
    }
}

impl Action for GivePawn {
    fn play(&mut self, game: &mut Game) -> Result<(), RulesError> {
        if game.player_pawn_count(self.player_id) >= game.settings.max_pawn_per_player {
            Err(RulesError::MaximumPawnCount)
        } else {
            match game.get_pawn_mut(self.pawn_id) {
                Some(pawn) => {
                    self.previous_owner = pawn.owner_id;
                    pawn.owner_id = Some(self.player_id);
                    Ok(())
                },
                None => Err(RulesError::PawnNotExists)
            }
        }
    }

    fn unplay(&mut self, game: &mut Game) {
        if let Some(player_id) = self.previous_owner {
            let _ = GivePawn::new(player_id, self.player_id).play(game);
        } else {
            if let Some(pawn) = game.get_pawn_mut(self.pawn_id) {
                pawn.owner_id = self.previous_owner;
            }
        }
    }
}