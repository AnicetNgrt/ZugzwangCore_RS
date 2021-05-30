use super::{Brain, Game};

pub struct IdleBrain();

impl Brain for IdleBrain {
    fn play(&mut self, game: &Game) {}
}