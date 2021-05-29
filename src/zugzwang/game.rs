use std::convert::TryFrom;
use std::collections::HashMap;

use super::{Pawn, Pawns, PawnState, Id, Size, Pacman, RulesError};

pub struct GameSettings {
    pub height: Size,
    pub width: Size,
    pub max_pawn_per_player: usize
}

pub struct Game<'a> {
    pub settings: &'a GameSettings,
    pub pawns: Pawns,
    pub players_ap: HashMap<Id, u8>
}

impl<'a> Game<'a> {
    pub fn new(settings: &'a GameSettings) -> Self {
        Game {
            settings: settings,
            pawns: HashMap::new(),
            players_ap: HashMap::new()
        }
    }

    pub fn new_pacman(&self, x: Size, y: Size) -> Pacman {
        Pacman::new(x, y, self.settings.width, self.settings.height)
    }

    fn set_state(&self, pawn: &mut Pawn, state: PawnState) -> Result<(), RulesError> {
        match (pawn.state, state) {
            (PawnState::Unplaced, PawnState::Unplaced) => Ok(()),
            (PawnState::Unplaced, PawnState::Placed(position)) => {
                if !self.is_position_ofb(&position) {
                    pawn.state = PawnState::Placed(position);
                    return Ok(());
                }
                if !self.is_position_free(&position) {
                    return Err(RulesError::PositionTaken);
                }
                pawn.state = state;
                Ok(())
            },
            _ => Err(RulesError::IllegalStateTransition)
        }
    }

    pub fn placed_pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.pawns.values()
            .filter(
                |&pawn| matches!(pawn.state, PawnState::Placed(_))
            )
    }

    pub fn unplaced_pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.pawns.values()
            .filter(
                |&pawn| matches!(pawn.state, PawnState::Unplaced)
            )
    }

    pub fn who_owns_pawn(&self, pawn_id: Id) -> Option<Id> {
        match self.pawns.get(&pawn_id) {
            Some(pawn) => pawn.owner_id,
            _err => None
        }
    }

    pub fn player_pawns(&self, player_id: Id) -> impl Iterator<Item = &Pawn> {
        self.pawns.values()
            .filter(move |&pawn| {
                if let Some(id) = pawn.owner_id {
                    id == player_id
                } else {
                    false
                }
            })
    }

    pub fn player_pawn_count(&self, player_id: Id) -> usize {
        self.player_pawns(player_id).count()
    }

    pub fn get_pawn_mut(&mut self, pawn_id: Id) -> Option<&mut Pawn> {
        self.pawns.get_mut(&pawn_id)
    }

    fn give_pawn(&mut self, player_id: Id, pawn: &mut Pawn) {
        pawn.owner_id = Some(player_id);
    }

    fn is_position_ofb(&self, position: &Pacman) -> bool {
        position.x < self.settings.width && position.y < self.settings.height
    }

    fn is_position_free(&self, position: &Pacman) -> bool {
        let mut found = false;
        for p in self.pawns.values() {
            found = match p.state {
                PawnState::Placed(pawn_position) => 
                    position.equals(pawn_position),
                _ => false
            };
            if found {
                break;
            }
        };
        !found
    }

    pub fn gen_pawn_id(&self) -> Id {
        Id::try_from(self.pawns.values().count()).unwrap()
    }
}

#[cfg(test)]
mod tests {}