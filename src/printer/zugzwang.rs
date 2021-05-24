use super::core::*;
use crate::zugzwang::{Game, PawnState, Id};

pub fn print_game(game: &Game) {
    println!("{}", game.to_canvas())
}

impl Drawable for Game {
    fn to_canvas(&self) -> Canvas {
        board(self)
    }
}

fn board_header(game: &Game) -> Canvas {
    let mut canvas = Canvas::new(usize::from(game.width * 5), 1);
    for x in 0..game.width {
        canvas.put(usize::from(x*5), 0, format!("{:01$} ", x, 4));
    }
    canvas
}

fn board_body(game: &Game) -> Canvas {
    let mut canvas = Canvas::new(
        usize::from((game.width*5)+1),
        usize::from((game.height*2)+1)
    );
    for y in 0..game.height {
        for x in 0..game.width {
            canvas.stamp(
                &board_cell(), 
                usize::from(x*5), 
                usize::from(y*2)
            );
        }
    }
    put_placed_pawns(game, &mut canvas);
    canvas
}

fn pawn_to_string(owner_id: Option<&Id>, pawn_id: Id) -> String {
    match owner_id {
        Some(owner_id) => {
            let pawns = vec![
                vec!["🐢", "🐸", "🐍", "🐲", "🐊", "🦎", "👽", "🥦", "🥑"],
                vec!["🐠", "🦐", "🐙", "🦀", "🐜", "🦉", "🍘", "🥕", "🥭"]
            ];

            pawns[*owner_id as usize][pawn_id as usize].to_owned()
        },
        None => "😎".to_owned()
    }
}

fn put_placed_pawns(game: &Game, board_canvas: &mut Canvas) {
    for pawn in game.placed_pawns() {
        match pawn.state {
            PawnState::Placed{x, y} => {
                let pawn_string = pawn_to_string(game.who_owns_pawn(pawn.id), pawn.id);
                board_canvas.put(((x*5)+2) as usize, ((y*2)+1) as usize, pawn_string);
                board_canvas.put(((x*5)+3) as usize, ((y*2)+1) as usize, "");
            }
            _ => ()
        }
    }
}

fn board_cell() -> Canvas {
    let mut canvas = Canvas::new(6, 3);
    canvas.put(0, 0, "o");
    canvas.put(5, 0, "o");
    for x in 1..5 { canvas.put(x, 0, "-"); }
    canvas.put(0, 2, "o");
    canvas.put(5, 2, "o");
    for x in 1..5 { canvas.put(x, 2, "-"); }
    canvas.put(0, 1, "|");
    canvas.put(5, 1, "|");
    canvas
}

fn player_unplaced_pawns(game: &Game, player_id: Id) -> Canvas {
    let mut canvas = Canvas::new(4, 19);
    let pawns_iterator = game.unplaced_pawns()
        .filter(|&pawn| match game.who_owns_pawn(pawn.id) {
            Some(id) => *id == player_id,
            _ => false
        });
    canvas.put(1, 0, format!("p{}", player_id));
    canvas.put(2, 0, "");
    let mut i = 1;
    for pawn in pawns_iterator {
        canvas.put(1, i, pawn_to_string(Some(&player_id), pawn.id));
        canvas.put(2, i, "");
        i += 1;
    }
    canvas.add_borders()
}

fn board(game: &Game) -> Canvas {
    let header = board_header(game);
    let mut canvas = Canvas::new(125, 22);
    canvas.stamp(&header, 0, 0);
    canvas.stamp(&board_body(game), 0, 1);
    canvas.put_splitted(77, 0, "Unplaced pawns");
    canvas.stamp(&player_unplaced_pawns(game, 0), 78, 1);
    canvas.stamp(&player_unplaced_pawns(game, 1), 85, 1);
    canvas.add_padding(3, 3, 1, 1)
}