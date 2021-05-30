use super::{CreatePawn, GivePawn};
use super::super::{Action, Id, Game, RulesError};

pub struct CreateGivePawn {
    pub player_id: Id,
    create_pawn: CreatePawn,
    give_pawn: GivePawn,
}

impl CreateGivePawn {
    pub fn new(player_id: Id) -> Self {
        CreateGivePawn {
            player_id,
            create_pawn: CreatePawn::new(),
            give_pawn: GivePawn::new(player_id, 0)
        }
    }
}

impl Action for CreateGivePawn {
    fn play(&mut self, game: &mut Game) -> Result<(), RulesError> {
        match self.create_pawn.play(game) {
            Ok(()) => {
                self.give_pawn.pawn_id = self.create_pawn.pawn_id;
                self.give_pawn.play(game)
            },
            err => err
        }
    }

    fn unplay(&mut self, game: &mut Game) -> Result<(), RulesError> {
        match self.give_pawn.unplay(game) {
            Ok(()) => {
                self.create_pawn.unplay(game)
            },
            err => err
        }
    }
}