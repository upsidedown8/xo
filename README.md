# xo
Naughts and crosses / Tic-tac-toe AI in Rust.

Uses the following negamax algorithm (with alpha beta pruning) to play perfect games.

```rust

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
```
