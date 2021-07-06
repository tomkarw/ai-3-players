use std::fmt::{Display, Formatter, Result};
use termion::color;

// const EMPTY_TILE: u8 = 0;
const BG_GREEN: color::Bg<color::Green> = color::Bg(color::Green);
const BG_BLACK: color::Bg<color::Black> = color::Bg(color::Black);
const BG_WHITE: color::Bg<color::White> = color::Bg(color::White);
const BG_RED: color::Bg<color::Red> = color::Bg(color::Red);
const RST_CLR: color::Bg<color::Reset> = color::Bg(color::Reset);

/// Represents game board and it's state
pub(crate) struct BoardState {
    rows: usize,
    columns: usize,
    board: Vec<i8>,
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
}
