# xo
Naughts and crosses / Tic tac toe AI in Rust.

Uses the following negamax algorithm to play perfect games.

```rust
fn negamax(board: &mut Board) -> i32 {
    let next_player = board.next_player();

    match board.state() {
        GameState::Winner(player) if player == next_player => 1,
        GameState::Winner(_) => -1,
        GameState::Draw => 0,
        GameState::Indeterminate => {
            let mut best_score = i32::MIN;

            for pos in 0..9 {
                if board.get_square(pos).is_none() {
                    board.set_square(pos, Some(next_player));
                    best_score = best_score.max(-negamax(board));
                    board.set_square(pos, None);
                }
            }

            best_score
        }
    }
}
```
