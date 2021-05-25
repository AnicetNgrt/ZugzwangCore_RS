use super::core::*;
use crate::zugzwang::core::{Game, PawnState, Id, Size, Position};

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn print_game(game: &Game) {
    println!(" ZUGZWANG ♟{}", game.to_canvas())
}

impl Drawable for Game {
    fn to_canvas(&self) -> Canvas {
        board(self)
    }
}

fn board_body(game: &Game, show_coords: bool) -> Canvas {
    let mut canvas = Canvas::new(
        usize::from((game.width*5)+1),
        usize::from((game.height*2)+1)
    );
    for y in 0..game.height {
        for x in 0..game.width {
            canvas.stamp(
                &board_cell(x, y, game.width, game.height, show_coords), 
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
                vec![
                    "🐢", "🐸", "🐍", "🐲",
                    "🐊", "🦎", "👽", "🥦",
                    "🥑", "🧶", "🎄", "🧩",
                    "📗", "💚", "🟢", "🟩"
                ],
                vec![
                    "🐠", "🦐", "🐙", "🦀",
                    "🐜", "🦉", "🍘", "🥕",
                    "🥭", "🧨", "🧧", "🎃",
                    "🟧", "🟠", "🏓", "🏀"
                ]
            ];

            pawns[*owner_id as usize][pawn_id as usize].to_owned()
        },
        None => "😎".to_owned()
    }
}

fn put_placed_pawns(game: &Game, board_canvas: &mut Canvas) {
    for pawn in game.placed_pawns() {
        if let PawnState::Placed(Position{x, y}) = pawn.state {
            let pawn_string = pawn_to_string(game.who_owns_pawn(pawn.id), pawn.id);
            board_canvas.put(((x*5)+2) as usize, ((y*2)+1) as usize, pawn_string);
            board_canvas.put(((x*5)+3) as usize, ((y*2)+1) as usize, "");
        }
    }
}

fn board_cell(x: Size, y: Size, width: Size, height: Size, show_coords: bool) -> Canvas { 
    let mut canvas = Canvas::new(6, 3);
    
    // Drawing borders
    for x in 1..5 { canvas.put(x, 0, "─"); }
    for x in 1..5 { canvas.put(x, 2, "─"); }
    canvas.put(0, 1, "│");
    canvas.put(5, 1, "│");

    // Calculating borders
    let t_border = y == 0;
    let b_border = y == height-1;
    let l_border = x == 0;
    let r_border = x == width-1;
    let borders = (t_border, b_border, l_border, r_border);

    // Drawing corners
    let mut corners = vec!["┼", "┼", "┼", "┼"];
    if let (true, _, false, _) = borders {
        corners[0] = "┬";
        corners[1] = "┬";
    }
    if let (_, true, _, _) = borders {
        corners[2] = "┴";
        corners[3] = "┴";
    }
    if let (true, _, false, _) = borders { corners[0] = "┬" }
    if let (true, _, _, false) = borders { corners[1] = "┬" }
    if let (_, true, false, _) = borders { corners[2] = "┴" }
    if let (_, true, _, false) = borders { corners[3] = "┴" }
    if let (true, _, true, _) = borders { corners[0] = "┌" }
    if let (true, _, _, true) = borders { corners[1] = "┐" }
    if let (_, true, true, _) = borders { corners[2] = "└" }
    if let (_, true, _, true) = borders { corners[3] = "┘" }
    if let (false, _, true, _) = borders { corners[0] = "├" }
    if let (_, false, true, _) = borders { corners[2] = "├" }
    if let (false, _, _, true) = borders { corners[1] = "┤" }
    if let (_, false, _, true) = borders { corners[3] = "┤" }
    canvas.put(0, 0, corners[0]);
    canvas.put(5, 0, corners[1]);
    canvas.put(0, 2, corners[2]);
    canvas.put(5, 2, corners[3]);
    
    // Drawing coordinates
    if show_coords {
        if t_border {
            canvas.put_splitted(2, 0, format!("{}", x));
        }
        if l_border {
            canvas.put(0, 1, ALPHA.chars().nth(y as usize).unwrap_or('?'));
        }
    }

    canvas
}

fn player_unplaced_pawns(game: &Game, player_id: Id) -> Canvas {
    let mut canvas = Canvas::new(2, 1);
    canvas.put_splitted(0, 0, format!("p{}", player_id));
    canvas = canvas.add_padding(1, 1, 0, 0);

    let pawns_iterator = game.unplaced_pawns()
        .filter(|&pawn| match game.who_owns_pawn(pawn.id) {
            Some(id) => *id == player_id,
            _ => false
        });

    for pawn in pawns_iterator {
        let mut pawn_canvas = Canvas::new(2, 1);
        pawn_canvas.put(0, 0, pawn_to_string(Some(&player_id), pawn.id));
        pawn_canvas.put(1, 0, "");
        canvas = canvas.add_right(&pawn_canvas).add_padding(0, 1, 0, 0);
    }
    
    canvas.add_borders()
}

fn board(game: &Game) -> Canvas {
    let board_body = board_body(game, false);
    let unplaced_pawns = 
        player_unplaced_pawns(game, 0)
        .add_right(&player_unplaced_pawns(game, 1));
    let body = board_body.add_bottom(&unplaced_pawns);
    body.add_padding(1, 1, 0, 0)
}