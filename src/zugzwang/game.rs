use std::collections::HashMap;

use super::{Pawn, PawnState, Id, Size, Pacman, RulesError, Player, Brain, Action};
use super::actions::{CreateGivePawn};

pub struct GameSettings {
    pub height: Size,
    pub width: Size,
    pub player_count: usize,
    pub end_turn_ap_gain: u8,
    pub max_ap_per_player: u8,
    pub start_pawn_per_player: usize,
    pub max_pawn_per_player: usize
}

pub struct Game<'a> {
    pub turn: u8,
    pub settings: &'a GameSettings,
    pub pawns: HashMap<Id, Pawn>,
    pub players: HashMap<Id, Player>
}

impl<'a> Game<'a> {
    pub fn new(settings: &'a GameSettings) -> Self {
        Game {
            turn: 0,
            settings: settings,
            pawns: HashMap::new(),
            players: HashMap::new()
        }
    }

    pub fn after_turn(&mut self) {
        self.turn += 1;
    }

    pub fn before_turn(&mut self) {
        if self.turn == 0 {
            self.first_turn_init_players();
            self.first_turn_init_pawns();
        }
    }

    fn first_turn_init_players(&mut self) {
        for _ in 0..self.settings.player_count {
            self.add_player();
        }
    }

    fn first_turn_init_pawns(&mut self) {
        for player_id in self.get_player_ids() {
            for _ in 0..self.settings.start_pawn_per_player {
                CreateGivePawn::new(player_id).play(self).unwrap();
            }
        }
    }

    pub fn get_player_ids(&self) -> Vec<Id> {
        self.players.keys().map(|&id| id).collect()
    }

    pub fn add_player(&mut self) {
        let id = self.gen_player_id();
        self.players.insert(id, Player { id, ap: 0 });
    }

    pub fn new_pacman(&self, x: Size, y: Size) -> Pacman {
        Pacman::new(x, y, self.settings.width, self.settings.height)
    }

    fn set_state(&self, pawn: &mut Pawn, state: PawnState) -> Result<(), RulesError> {
        match (pawn.state, state) {
            (PawnState::Unplaced, PawnState::Unplaced) => Ok(()),
            (PawnState::Unplaced, PawnState::Placed(position)) => {
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

    pub fn is_position_free(&self, position: &Pacman) -> bool {
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
        let mut max: Option<Id> = None;
        for pawn in self.pawns.values() {
            match max {
                None => max = Some(pawn.id),
                Some(id) => if pawn.id > id {
                    max = Some(pawn.id);
                }
            }
        }
        max.and_then(|id| Some(id+1)).unwrap_or(0)
    }

    pub fn gen_player_id(&self) -> Id {
        let mut max: Option<Id> = None;
        for player in self.players.values() {
            match max {
                None => max = Some(player.id),
                Some(id) => if player.id > id {
                    max = Some(player.id);
                }
            }
        }
        max.and_then(|id| Some(id+1)).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {}