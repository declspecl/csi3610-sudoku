use std::fmt::Write;

use crate::board::{digit::Digit, digit_candidate_set::DigitCandidateSet, position::Position};

pub const BOARD_LENGTH: u8 = 9;
pub const BOARD_CELL_COUNT: u8 = BOARD_LENGTH * BOARD_LENGTH;

#[derive(Debug)]
pub struct Board {
    cells: [DigitCandidateSet; BOARD_CELL_COUNT as usize]
}

impl Board {
    const FULL_CELLS: [DigitCandidateSet; BOARD_CELL_COUNT as usize] = [DigitCandidateSet::ALL; BOARD_CELL_COUNT as usize];

    pub fn new() -> Self {
        return Self { cells: Self::FULL_CELLS };
    }

    pub fn at(&self, position: Position) -> DigitCandidateSet {
        return self.cells[position.id() as usize];
    }

    pub fn solve_cell(&mut self, position: Position, digit: Digit) {
        self.cells[position.id() as usize] = DigitCandidateSet::of(digit);

        for peer_id in position.peer_ids() {
            let cell = self.cells[peer_id as usize];
            self.cells[peer_id as usize] = cell.remove(digit);
        }
    }

    pub fn is_solved(&self) -> bool {
        return self.cells.iter()
            .all(|cell| cell.is_solved());
    }

    pub fn display_candidates(&self) -> String {
        let mut out = String::new();

        for board_row in 0..BOARD_LENGTH {
            let is_box_row_boundary = board_row > 0 && board_row % 3 == 0;
            let is_row_boundary = board_row > 0;

            if is_box_row_boundary {
                writeln!(out, "===+===+=== ++ ===+===+=== ++ ===+===+===").unwrap();
                writeln!(out, "").unwrap();
                writeln!(out, "===+===+=== ++ ===+===+=== ++ ===+===+===").unwrap();
            } else if is_row_boundary {
                writeln!(out, "---+---+--- ++ ---+---+--- ++ ---+---+---").unwrap();
            }

            for candidate_row in 0..3u8 {
                for board_col in 0..BOARD_LENGTH {
                    let is_box_col_boundary = board_col > 0 && board_col % 3 == 0;
                    let is_col_boundary = board_col > 0;

                    if is_box_col_boundary {
                        write!(out, " || ").unwrap();
                    } else if is_col_boundary {
                        write!(out, "|").unwrap();
                    }

                    let cell = self.cells[((board_row * BOARD_LENGTH) + board_col) as usize];

                    for candidate_col in 0..3u8 {
                        let digit = Digit::ALL[((candidate_row * 3) + candidate_col) as usize];
                        let is_digit_candidate = cell.contains(digit);

                        if is_digit_candidate {
                            write!(out, "{}", digit.as_u8()).unwrap();
                        } else {
                            write!(out, " ").unwrap();
                        }
                    }
                }

                writeln!(out).unwrap();
            }
        }

        return out;
    }

    pub fn display_solution(&self) -> String {
        let mut out = String::new();

        for board_row in 0..BOARD_LENGTH {
            let is_box_row_boundary = board_row > 0 && board_row % 3 == 0;
            if is_box_row_boundary {
                writeln!(out, "---+---+---").unwrap();
            }

            for board_col in 0..BOARD_LENGTH {
                let is_box_col_boundary = board_col > 0 && board_col % 3 == 0;
                if is_box_col_boundary {
                    write!(out, "|").unwrap();
                }

                let cell = self.cells[((board_row * BOARD_LENGTH) + board_col) as usize];
                if let Some(digit) = cell.solved_digit() {
                    write!(out, "{}", digit.as_u8()).unwrap();
                } else {
                    write!(out, "?").unwrap();
                }
            }

            writeln!(out).unwrap();
        }

        return out;
    }
}

