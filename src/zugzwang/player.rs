use super::{Game, Id};

#[derive(Debug)]
pub struct Player {
    pub ap: u8,
    pub id: Id
}

pub trait Brain {
    fn play(&mut self, game: &mut Game, player_id: Id);
}