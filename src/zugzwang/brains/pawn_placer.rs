use super::super::actions;
use super::super::{Brain, Game, Id, Pacman, Action};

pub struct DeterministicPawnPlacer {
    pub positions: Vec<Pacman>
}

impl Brain for DeterministicPawnPlacer {
    fn play(&mut self, game: &mut Game, player_id: Id) {
        let unplaced_pawns_ids: Vec<Id> = game.unplaced_pawns()
            .filter(|&pawn| {
                if let Some(id) = pawn.owner_id {
                    id == player_id
                } else {
                    false
                }
            })
            .map(|&pawn| pawn.id)
            .collect();
        
        for pawn_id in unplaced_pawns_ids.into_iter() {
            loop {
                if let Some(position) = self.positions.pop() {
                    let mut action = actions::PlacePawn::new(pawn_id, &position);
                    if let Ok(()) = action.play(game) {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}