mod printer;
mod zugzwang;

use crate::zugzwang::{GameSettings, Game, Action};
use crate::zugzwang::actions::CreateGivePawn;
use crate::printer::*;

fn main() {
    printer::clear();

    let mut game = Game::new(&GameSettings {
        width: 15,
        height: 10,
        max_pawn_per_player: 4
    });

    // let pawns = vec![
    //     (0, PawnState::Placed(game.new_pacman(15, 10))),
    //     (0, PawnState::Unplaced),
    //     (0, PawnState::Unplaced),
    //     (0, PawnState::Unplaced),
    //     (0, PawnState::Unplaced),
    //     (0, PawnState::Unplaced),
    //     (0, PawnState::Unplaced),
    //     (0, PawnState::Placed(game.new_pacman(0, 9))),
    //     (1, PawnState::Placed(game.new_pacman(4, 7))),
    //     (1, PawnState::Unplaced),
    //     (1, PawnState::Unplaced),
    //     (1, PawnState::Unplaced),
    //     (1, PawnState::Unplaced),
    //     (1, PawnState::Unplaced),
    //     (1, PawnState::Placed(game.new_pacman(14, 3))),
    //     (1, PawnState::Placed(game.new_pacman(1, 6)))
    // ];

    // game.create_pawns_from(pawns.iter()).unwrap();

    CreateGivePawn::new(0).play(&mut game);

    println!("{}", game.gen_pawn_id());
    for val in game.pawns.values() {
        println!("{:?}", val);
    }

    CreateGivePawn::new(1).play(&mut game);

    println!("{}", game.gen_pawn_id());
    for val in game.pawns.values() {
        println!("{:?}", val);
    }

    print_game(&game);

    loop {}
}
