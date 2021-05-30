use std::collections::HashMap;

mod printer;
mod zugzwang;

use crate::zugzwang::{GameSettings, Game, Id, Brain};
use crate::zugzwang::brains::IdleBrain;
use crate::printer::*;

fn main() {
    printer::clear();

    let mut game = Game::new(&GameSettings {
        width: 15,
        height: 10,
        player_count: 2,
        end_turn_ap_gain: 4,
        max_ap_per_player: 4,
        start_pawn_per_player: 3,
        max_pawn_per_player: 4,
    });

    let mut brains: HashMap<Id, Box<dyn Brain>> = HashMap::new();
    brains.insert(0, Box::new(IdleBrain()));
    brains.insert(1, Box::new(IdleBrain()));

    game.play_turn(&mut brains);

    // for val in game.pawns.values() {
    //     println!("{:?}", val);
    // }

    println!();
    print_game(&game);

    loop {}
}
