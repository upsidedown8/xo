use xo::{
    ai,
    board::{Board, GameState, Player},
    error::Error,
};

const AI_PLAYER: Option<Player> = Some(Player::X);

fn ai_move(board: &mut Board) {
    let best_move = ai::best_move(board).unwrap();
    board.make_move(best_move).unwrap();
}
fn user_move(board: &mut Board) {
    let mut buf = String::new();
    loop {
        println!("enter position: (0..9)");

        buf.clear();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read stdin");

        let pos = match buf.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("expected a valid unsigned integer");
                continue;
            }
        };

        match board.make_move(pos) {
            Ok(_) => break,
            Err(Error::InvalidPosition(_)) => println!("expected number in range 0..9"),
            Err(Error::Occupied(_)) => println!("square at index {} was occupied", pos),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut board = Board::default();

    while board.state() == GameState::Indeterminate {
        if AI_PLAYER == Some(board.next_player()) {
            ai_move(&mut board);
        } else {
            println!("\n\n{}\n{} to move", board, board.next_player());
            user_move(&mut board);
        }
    }

    println!("\n\n{}", board);

    match board.state() {
        GameState::Winner(player) => println!("{} has won", player),
        GameState::Draw => println!("draw"),
        _ => unreachable!(),
    };
}
