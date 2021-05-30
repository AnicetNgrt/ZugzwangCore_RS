use super::{Game, Id, Action};

#[derive(Debug)]
pub struct Player {
    pub ap: u8,
    pub id: Id
}

pub trait Brain {
    fn play(&mut self, game: &mut Game, player_id: Id);
}