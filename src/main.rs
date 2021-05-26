mod printer;
mod zugzwang;

use crate::zugzwang::{Game, PawnState};
use crate::printer::*;

fn main() {
    printer::clear();

    let mut game = Game::new(15, 10);

    let pawns = vec![
        (0, PawnState::Placed(game.new_pacman(15, 10))),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Placed(game.new_pacman(0, 9))),
        (1, PawnState::Placed(game.new_pacman(4, 7))),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Placed(game.new_pacman(14, 3))),
        (1, PawnState::Placed(game.new_pacman(1, 6)))
    ];

    game.create_pawns_from(pawns.iter()).unwrap();

    print_game(&game);

    loop {}
}
