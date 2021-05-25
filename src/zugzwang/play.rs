use super::core::{Game, Size, Position};

pub enum Action {
    PlacePawn{
        pawn_id: Size,
        position: Position
    }
}

pub trait Player {
    fn play(&mut self, game: &Game) -> Action;
}