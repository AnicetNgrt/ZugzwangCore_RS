use super::{Game, Action};

pub trait Player {
    fn play(&mut self, game: &Game, actions: Vec<Box<dyn Action>>) -> u64;
}