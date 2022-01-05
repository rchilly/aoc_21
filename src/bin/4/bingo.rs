use std::fmt::{Result as FmtResult, *};
use std::result::Result as StdResult;

const ROWS: usize = 5;
const COLUMNS: usize = 5;

#[derive(Clone)]
pub struct Board {
    rows: [[u8; COLUMNS]; ROWS],
    marks: Vec<usize>,
    pub bingo: bool,
}

#[derive(Debug)]
pub enum Outcome {
    NoMark,
    Marked,
    Bingo,
}

impl Board {
    pub fn new(numbers: &[u8]) -> StdResult<Self, String> {
        if numbers.len() != ROWS * COLUMNS {
            return Err(format!(
                "numbers must evenly fill {} rows of {} each",
                ROWS, COLUMNS
            ));
        }

        let mut rows = [[0u8; COLUMNS]; ROWS];
        for (i, row) in rows.iter_mut().enumerate() {
            let from = i * ROWS;
            let to = from + COLUMNS;

            for (i, number) in (&numbers[from..to]).iter().enumerate() {
                row[i] = *number;
            }
        }

        Ok(Board {
            rows,
            marks: Vec::default(),
            bingo: false,
        })
    }

    pub fn play(&mut self, drawn: u8) -> Outcome {
        let mut outcome = Outcome::NoMark;
        'outer: for (i, row) in self.rows.iter().enumerate() {
            for (j, number) in row.iter().enumerate() {
                if *number == drawn {
                    let index = i * ROWS + j;
                    self.marks.push(index);
                    outcome = Outcome::Marked;
                    break 'outer;
                }
            }
        }

        if matches!(outcome, Outcome::Marked) && self.has_bingo() {
            outcome = Outcome::Bingo;
            self.bingo = true;
        }

        outcome
    }

    pub fn score(&self, last_drawn: u8) -> usize {
        if !self.has_bingo() {
            return 0;
        }

        let mut unmarked: usize = 0;
        for (ridx, row) in self.rows.iter().enumerate() {
            for (cidx, number) in row.iter().enumerate() {
                if !self.is_marked(ridx, cidx) {
                    unmarked += *number as usize;
                }
            }
        }

        unmarked * last_drawn as usize
    }

    fn has_bingo(&self) -> bool {
        let mut bingo = false;

        for cidx in 0..COLUMNS {
            bingo = true;

            for (ridx, _) in self.rows.iter().enumerate() {
                if !self.is_marked(ridx, cidx) {
                    bingo = false;
                    break;
                }
            }

            if bingo {
                return bingo;
            }
        }

        for (ridx, row) in self.rows.iter().enumerate() {
            bingo = true;

            for (cidx, _) in row.iter().enumerate() {
                if !self.is_marked(ridx, cidx) {
                    bingo = false;
                    break;
                }
            }

            if bingo {
                return bingo;
            }
        }

        bingo
    }

    fn is_marked(&self, row_idx: usize, col_idx: usize) -> bool {
        let idx = row_idx * ROWS + col_idx;
        self.marks.contains(&idx)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut display = String::new();
        writeln!(display)?;
        for (ridx, row) in self.rows.iter().enumerate() {
            for (cidx, number) in row.iter().enumerate() {
                if self.is_marked(ridx, cidx) {
                    write!(display, "{:3} ", "X ")?;
                } else {
                    write!(display, "{:<3} ", number)?;
                }
            }

            display.pop();
            display.push('\n');
        }

        f.write_str(&display)
    }
}
