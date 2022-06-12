use crate::{
    board::{Board, GameState},
    error::{Error, Result},
};

const POS_INF: i32 = i32::MAX;
const NEG_INF: i32 = i32::MIN + 1;

fn negamax(board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
    let next_player = board.next_player();

    match board.state() {
        GameState::Winner(player) if player == next_player => 1,
        GameState::Winner(_) => -1,
        GameState::Draw => 0,
        GameState::Indeterminate => {
            let mut best_score = NEG_INF;

            for pos in 0..9 {
                if board.get_square(pos).is_none() {
                    board.set_square(pos, Some(next_player));
                    best_score = best_score.max(-negamax(board, -beta, -alpha));
                    board.set_square(pos, None);

                    alpha = alpha.max(best_score);

                    if alpha >= beta {
                        break;
                    }
                }
            }

            best_score
        }
    }
}

/// Finds the best move in a given position
///
/// # Arguments
/// * `board` The position to search from
pub fn best_move(board: &Board) -> Result<usize> {
    if !board.is_valid() {
        Err(Error::InvalidBoard)
    } else if board.state() != GameState::Indeterminate {
        Err(Error::GameOver)
    } else {
        let mut board = board.clone();

        let mut best_score = NEG_INF;
        let mut best_move = None;

        for pos in 0..9 {
            if board.get_square(pos).is_none() {
                board.set_square(pos, Some(board.next_player()));
                let score = -negamax(&mut board, NEG_INF, POS_INF);
                board.set_square(pos, None);
                if score >= best_score {
                    best_score = score;
                    best_move = Some(pos);
                }
            }
        }

        best_move.ok_or(Error::GameOver)
    }
}
