use crate::error::{Error, Result};
use std::{convert::TryFrom, fmt::Display};

/// Stores the positions of X and O on the board.
#[derive(Default, Clone)]
pub struct Board {
    /// 9 bits are required so a 16 bit unsigned type
    /// is needed. Stores positions for Player::X
    squares_x: u16,
    /// 9 bits are required so a 16 bit unsigned type
    /// is needed. Stores positions for Player::O
    squares_o: u16,
}

impl Board {
    /// Which player needs to move next?
    pub fn next_player(&self) -> Player {
        if self.squares_x.count_ones() > self.squares_o.count_ones() {
            Player::O
        } else {
            Player::X
        }
    }

    /// Gets the current `GameState` (Win / Draw / Indeterminate)
    ///
    pub fn state(&self) -> GameState {
        #[allow(clippy::unusual_byte_groupings)]
        const WINS: [u16; 8] = [
            0b111_000_000,
            0b000_111_000,
            0b000_000_111,
            0b001_001_001,
            0b010_010_010,
            0b100_100_100,
            0b100_010_001,
            0b001_010_100,
        ];

        for win in WINS {
            if self.squares_o & win == win {
                return GameState::Winner(Player::O);
            }
            if self.squares_x & win == win {
                return GameState::Winner(Player::X);
            }
        }

        if self.squares_o | self.squares_x == 0b111111111 {
            return GameState::Draw;
        }

        GameState::Indeterminate
    }

    /// Gets the player at `pos` on the board. Returns `None` if
    /// the square is outside the range (0..9) or if the square is empty.
    ///
    /// # Arguments
    /// * `pos` The position on the board (0..9)
    pub fn get_square(&self, pos: usize) -> Option<Player> {
        match pos {
            0..=8 if 0 != self.squares_x & (1 << pos) => Some(Player::X),
            0..=8 if 0 != self.squares_o & (1 << pos) => Some(Player::O),
            _ => None,
        }
    }

    /// Makes a move with `self.next_player` at position `pos`
    ///
    /// # Arguments
    /// * `pos` The position on the board (0..9)
    pub fn make_move(&mut self, pos: usize) -> Result<GameState> {
        let state = self.state();
        let ongoing = state == GameState::Indeterminate;

        match pos {
            0..=8 if ongoing && self.get_square(pos).is_none() => {
                *match self.next_player() {
                    Player::X => &mut self.squares_x,
                    Player::O => &mut self.squares_o,
                } |= 1 << pos;
                Ok(state)
            }
            0..=8 if ongoing => Err(Error::Occupied(pos)),
            0..=8 => Err(Error::GameOver),
            _ => Err(Error::InvalidPosition(pos)),
        }
    }

    /// Sets the square at `pos` to `val`. Does not check whether the
    /// move is legal.
    ///
    /// # Arguments
    ///
    /// * `pos` The position of the square
    /// * `val` None => Empty, Some(v) => v
    pub fn set_square(&mut self, pos: usize, val: Option<Player>) {
        if (0..=8).contains(&pos) {
            let bit = 1 << pos;
            self.squares_o &= !bit;
            self.squares_x &= !bit;

            match val {
                Some(Player::X) => self.squares_x |= bit,
                Some(Player::O) => self.squares_o |= bit,
                None => (),
            }
        }
    }

    /// Checks whether `self` is a valid position
    pub fn is_valid(&self) -> bool {
        self.squares_o & self.squares_x == 0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            write!(f, "+---+---+---+\n| ")?;
            for col in 0..3 {
                let pos = row * 3 + col;
                match self.get_square(pos) {
                    Some(player) => write!(f, "{} | ", player)?,
                    None => write!(f, "  | ")?,
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "+---+---+---+")?;

        Ok(())
    }
}
impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut board = Board::default();
        let mut pos = 0;

        for ch in value.chars() {
            match ch {
                'O' | '0' | 'o' => board.squares_o |= 1 << pos,
                'X' | 'x' => board.squares_x |= 1 << pos,
                ' ' => (),
                _ => pos -= 1,
            }

            if pos >= 9 {
                return Err(Error::InvalidBoardLength);
            }

            pos += 1;
        }

        Ok(board)
    }
}

/// A player in XO
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    /// Player 1
    X,
    /// Player 2
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    Winner(Player),
    Draw,
    Indeterminate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let board = Board::try_from("oxo-xox-ooo").unwrap();
        assert_eq!(board.to_string(), "+---+---+---+\n| O | X | O | \n+---+---+---+\n| X | O | X | \n+---+---+---+\n| O | O | O | \n+---+---+---+\n");
    }

    #[test]
    fn win_x() {
        let board = Board::try_from("oox-xxx-o  ").unwrap();
        assert_eq!(board.state(), GameState::Winner(Player::X))
    }

    #[test]
    fn win_o() {
        let board = Board::try_from("xxo-oxo-x o").unwrap();
        assert_eq!(board.state(), GameState::Winner(Player::O))
    }

    #[test]
    fn draw() {
        let board = Board::try_from("xox-xoo-oxx").unwrap();
        assert_eq!(board.state(), GameState::Draw)
    }

    #[test]
    fn indeterminate() {
        let board = Board::try_from("x o-oxx-xoo").unwrap();
        assert_eq!(board.state(), GameState::Indeterminate)
    }

    #[test]
    fn game_start() {
        let board = Board::default();
        assert_eq!(board.state(), GameState::Indeterminate);
    }
}
