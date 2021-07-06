use itertools::iproduct;
use std::fmt::{Display, Formatter, Result};
use termion::color;

const EMPTY_TILE: u8 = 0;
const BG_GREEN: color::Bg<color::Green> = color::Bg(color::Green);
const BG_BLACK: color::Bg<color::Black> = color::Bg(color::Black);
const BG_WHITE: color::Bg<color::White> = color::Bg(color::White);
const BG_RED: color::Bg<color::Red> = color::Bg(color::Red);
const RST_CLR: color::Bg<color::Reset> = color::Bg(color::Reset);

/// Represents game board and it's state
pub(crate) struct BoardState {
    rows: usize,
    columns: usize,
    board: Vec<u8>,
}

impl Display for BoardState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // column numbering
        write!(f, " ")?;
        for i in 0..self.columns {
            write!(f, "   {}", i)?;
        }
        writeln!(f)?;
        // top line
        write!(f, "  {}+ ", BG_GREEN)?;
        for _ in 0..self.columns - 1 {
            write!(f, "-   ")?;
        }
        writeln!(f, "- +{}", RST_CLR)?;
        // iterate over all rows
        for i in 0..self.rows {
            write!(f, "{} {}|", i, BG_GREEN)?;
            for j in 0..self.columns {
                match self.board[i * self.columns + j] {
                    1 => write!(f, "{}", BG_BLACK)?,
                    2 => write!(f, "{}", BG_WHITE)?,
                    3 => write!(f, "{}", BG_RED)?,
                    _ => {}
                };
                write!(f, "   {}|", BG_GREEN)?;
            }
            writeln!(f, "{}", RST_CLR)?;
            if i != self.rows - 1 {
                write!(f, "  {}  ", BG_GREEN)?;
                for _ in 0..self.columns - 1 {
                    write!(f, "-   ")?;
                }
                writeln!(f, "-  {}", RST_CLR)?;
            }
        }
        // bottom line
        write!(f, "  {}+ ", BG_GREEN)?;
        for _ in 0..self.columns - 1 {
            write!(f, "-   ")?;
        }
        writeln!(f, "- +{}", RST_CLR)?;
        Ok(())
    }
}

impl BoardState {
    pub(crate) fn new(rows: usize, columns: usize) -> Self {
        BoardState {
            rows,
            columns,
            board: vec![0; rows * columns],
        }
    }

    /// Create starting position for 3 player game
    pub(crate) fn setup_three_players(&mut self) {
        self.board[3 * self.columns + 3] = 1;
        self.board[3 * self.columns + 5] = 1;
        self.board[4 * self.columns + 4] = 1;
        self.board[3 * self.columns + 4] = 2;
        self.board[5 * self.columns + 3] = 2;
        self.board[5 * self.columns + 5] = 2;
        self.board[4 * self.columns + 3] = 3;
        self.board[4 * self.columns + 5] = 3;
        self.board[5 * self.columns + 4] = 3;
    }

    pub(crate) fn place(&mut self, row: usize, column: usize, player: u8) {
        for (row, column) in self.would_flip(row, column, player) {
            self.board[row * self.columns + column] = player;
        }
        self.board[row * self.columns + column] = player;
    }

    fn would_flip(&self, row: usize, column: usize, player: u8) -> Vec<(usize, usize)> {
        let row = row as i32;
        let column = column as i32;
        let mut flips = Vec::new();
        // iterate over possible directions
        for (dy, dx) in iproduct!(0..3, 0..3) {
            let dy: i32 = dy - 1;
            let dx: i32 = dx - 1;
            let mut visited_discs = Vec::new();
            if dy == 0 && dx == 0 {
                continue;
            }
            let mut current_row = row + dy;
            let mut current_column = column + dx;
            // stop when we reach board border
            while !(current_row < 0
                || current_row >= self.rows as i32
                || current_column < 0
                || current_column >= self.columns as i32)
            {
                match self.board[current_row as usize * self.columns + current_column as usize] {
                    // we found outflanking disc, append visited disc to flip list
                    x if x == player => {
                        flips.append(&mut visited_discs);
                        break;
                    }
                    // reached blank space, no discs could be outflanked
                    x if x == EMPTY_TILE => break,
                    // add met disc to visited discs
                    _ => visited_discs.push((current_row as usize, current_column as usize)),
                }
                current_row += dy;
                current_column += dx;
            }
        }
        flips
    }

    fn validate_placing(&self, row: usize, column: usize, player: u8) {
        todo!()
    }

    pub(crate) fn has_valid_move(&self, turn: usize) -> bool {
        todo!()
    }

    pub(crate) fn get_winner(&self) -> u8 {
        todo!()
    }
}
