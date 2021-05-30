use std::collections::HashMap;
use text_io::read;

mod printer;
mod zugzwang;

use crate::zugzwang::{GameSettings, Game, Id, Brain};
use crate::zugzwang::brains::{DeterministicPawnPlacer, IdleBrain};
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
    brains.insert(0, Box::new(DeterministicPawnPlacer {
        positions: vec![
            game.new_pacman(10, 8),
            game.new_pacman(5, 7),
            game.new_pacman(9, 3)
        ]
    }));
    brains.insert(1, Box::new(DeterministicPawnPlacer {
        positions: vec![
            game.new_pacman(9, 8),
            game.new_pacman(2, 9),
            game.new_pacman(12, 6)
        ]
    }));

    for _ in 0..u8::MAX {
        println!("------------Turn {}/{}-----------", game.turn, u8::MAX);
        play_turn(&mut game, &mut brains);
    }

    loop {}
}

fn play_turn(game: &mut Game, brains: &mut HashMap<Id, Box<dyn Brain>>) {
    game.before_turn();

    let mut sorted_ids = game.get_player_ids();
    sorted_ids.sort();
    for player_id in sorted_ids {
        let brain = brains.get_mut(&player_id).unwrap();
        brain.play(game, player_id);
        
        println!("~~~~~ Player {} just played", player_id);
        print_game(&game);

        println!("Press [ENTER] to continue.");
        let _: String = read!("{}\n");
        printer::clear();
    }

    game.after_turn();
}
