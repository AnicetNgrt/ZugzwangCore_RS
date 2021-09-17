mod pawn_placer;
pub use pawn_placer::*;

use super::{Brain, Game, Id};

pub struct IdleBrain();

impl Brain for IdleBrain {
    fn play(&mut self, _game: &mut Game, _player_id: Id) {}
}
