pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Debug)]
pub enum Error {
    /// The position should be in the range 0..=8
    InvalidPosition(usize),

    /// The board position was already occupied
    Occupied(usize),

    /// The game was already over, so no more moves
    /// can be played
    GameOver,

    /// Too many chars were provided to `Board::try_from`
    InvalidBoardLength,

    /// The board position was illegal
    InvalidBoard,
}
