use super::{Action, Id, Pawn, PawnState, Game, RulesError};

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

struct GivePawn {
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

    fn unplay(&mut self, game: &mut Game) -> Result<(), RulesError> {
        if let Some(player_id) = self.previous_owner {
            GivePawn::new(player_id, self.player_id).play(game)
        } else {
            if game.player_pawn_count(self.player_id) <= 0 {
                Err(RulesError::MinimumPawnCount)
            } else {
                match game.get_pawn_mut(self.pawn_id) {
                    Some(pawn) => {
                        pawn.owner_id = self.previous_owner;
                        Ok(())
                    },
                    None => Err(RulesError::PawnNotExists)
                }
            }
        }
    }
}

struct CreatePawn {
    pawn_id: Id
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
