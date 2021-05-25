mod printer;
mod zugzwang;

use crate::zugzwang::*;
use crate::printer::zugzwang::*;

fn main() {
    printer::clear();

    let pawns = vec![
        (0, PawnState::Placed{x: 15, y: 10}),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Unplaced),
        (0, PawnState::Placed{x: 0, y: 9}),
        (1, PawnState::Placed{x: 4, y: 7}),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Unplaced),
        (1, PawnState::Placed{x: 14, y: 3}),
        (1, PawnState::Placed{x: 1, y: 6})
    ];

    let mut game = Game::new(15, 10);
    game.create_pawns_from(pawns.iter()).unwrap();

    print_game(&game);

    loop {}
}
