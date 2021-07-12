use std::fmt::{Display, Formatter, Result};

use itertools::{iproduct, max};
use termion::color;

const EMPTY_TILE: usize = 0;
const BG_GREEN: color::Bg<color::Green> = color::Bg(color::Green);
const BG_BLACK: color::Bg<color::Black> = color::Bg(color::Black);
const BG_WHITE: color::Bg<color::White> = color::Bg(color::White);
const BG_RED: color::Bg<color::Red> = color::Bg(color::Red);
const RST_CLR: color::Bg<color::Reset> = color::Bg(color::Reset);

pub(crate) enum Move {
    Valid,
    Invalid(&'static str),
}

/// Represents game board and it's state
#[derive(Debug, Clone)]
pub(crate) struct BoardState {
    pub board: Vec<usize>,
    rows: usize,
    columns: usize,
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

    pub(crate) fn get(&self, row: usize, column: usize) -> usize {
        self.board[row * self.columns + column]
    }

    pub(crate) fn set(&mut self, row: usize, column: usize, player: usize) {
        self.board[row * self.columns + column] = player;
    }

    pub(crate) fn place(
        &mut self,
        row: usize,
        column: usize,
        player: usize,
    ) -> Vec<((usize, usize), usize)> {
        let would_flip = self.would_flip(row, column, player);
        let mut flipped = Vec::with_capacity(would_flip.len());
        for (r, c) in would_flip {
            flipped.push(((r, c), self.board[r * self.columns + c]));
            self.board[r * self.columns + c] = player;
        }
        flipped.push(((row, column), self.board[row * self.columns + column]));
        self.board[row * self.columns + column] = player;

        flipped
    }

    fn would_flip(&self, row: usize, column: usize, player: usize) -> Vec<(usize, usize)> {
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

    /// Makes sure move is valid
    pub(crate) fn validate_placing(&self, row: usize, column: usize, player: usize) -> Move {
        // outside bounds
        if row >= self.rows || column >= self.columns {
            return Move::Invalid("Placement out of bounds.");
        }
        // square is taken
        if self.board[row * self.columns + column] != EMPTY_TILE {
            return Move::Invalid("Square taken.");
        }
        // square not adjacent to any current discs
        let mut is_adjacent = false;
        for r in row as i32 - 1..row as i32 + 2 {
            for c in column as i32 - 1..column as i32 + 2 {
                if r < 0 || r >= self.rows as i32 || c < 0 || c >= self.columns as i32 {
                    continue;
                }
                if r == row as i32 && c == column as i32 {
                    continue;
                }
                if self.board[r as usize * self.columns + c as usize] != EMPTY_TILE {
                    is_adjacent = true;
                    break;
                }
            }
            if is_adjacent {
                break;
            }
        }
        if !is_adjacent {
            return Move::Invalid("New disc must be adjacent to some existing one.");
        }

        if self.would_flip(row, column, player).is_empty() {
            return Move::Invalid("Move must flip at least one disk.");
        }

        Move::Valid
    }

    pub(crate) fn has_valid_move(&self, player: usize) -> bool {
        for row in 0..self.rows {
            for column in 0..self.columns {
                if let Move::Valid = self.validate_placing(row, column, player) {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn valid_moves(&self, player: usize) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for row in 0..self.rows {
            for column in 0..self.columns {
                if let Move::Valid = self.validate_placing(row, column, player) {
                    moves.push((row, column));
                }
            }
        }
        moves
    }

    /// Check if game ended and if so return winning player.
    /// If at least two players score the same, we call a draw.
    pub(crate) fn get_winner(&self) -> usize {
        let mut scores = vec![0, 0, 0];
        for &cell in self.board.iter() {
            scores[cell as usize - 1] += 1;
        }

        let max_score = max(scores.iter()).unwrap();

        if scores.iter().filter(|&x| x == max_score).count() > 1 {
            return 0;
        }

        for (i, score) in scores.iter().enumerate() {
            if score == max_score {
                return i + 1;
            }
        }

        unreachable!();
    }
}
