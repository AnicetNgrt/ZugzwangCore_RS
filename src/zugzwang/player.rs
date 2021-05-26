use super::{Game, Action};

pub trait Player {
    fn play(&mut self, game: &Game) -> Action;
}