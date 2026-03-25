use std::{fmt::Write, str::FromStr};

use crate::board::{digit_candidate_set::DigitCandidateSet, position::Position};

pub const BOARD_LENGTH: u8 = 9;
pub const BOARD_CELL_COUNT: u8 = BOARD_LENGTH * BOARD_LENGTH;

/// A 9x9 Sudoku board
#[derive(Debug, Clone)]
pub struct Board {
    cells: [DigitCandidateSet; BOARD_CELL_COUNT as usize],
    solved_count: u8,
}

impl Board {
    /// a board with all cells initialized to all possible candidates
    /// (the default initial state of a board)
    const FULL_CELLS: [DigitCandidateSet; BOARD_CELL_COUNT as usize] = [DigitCandidateSet::ALL; BOARD_CELL_COUNT as usize];

    pub fn new() -> Self {
        return Self { cells: Self::FULL_CELLS, solved_count: 0 };
    }

    pub fn at(&self, position: Position) -> DigitCandidateSet {
        return self.cells[position.id() as usize];
    }

    /// returns true if the resulting board is valid, and false if it is now invalid
    pub fn solve_cell(&mut self, position: Position, digit: u8) -> bool {
        // solve this position
        self.cells[position.id() as usize] = DigitCandidateSet::of(digit);
        self.solved_count += 1;

        // propagate the fact that its peers now have one less candidate
        for peer_id in position.peer_ids() {
            let peer_cell = self.cells[peer_id as usize];
            if !peer_cell.contains(digit) {
                continue;
            }

            let pruned_peer_cell = peer_cell.remove(digit);
            self.cells[peer_id as usize] = pruned_peer_cell;

            // if this cell has no more candidates, the board is now invalid
            if pruned_peer_cell.is_empty() {
                return false;
            }

            // if this made the peer cell now have only 1 candidate, solve it
            if let Some(solved_peer_digit) = pruned_peer_cell.solved_digit() && !peer_cell.is_solved() {
                // if solving the peer cell results in a contradiction, the board is now invalid
                if !self.solve_cell(Position::from_id(peer_id), solved_peer_digit) {
                    return false;
                }
            }
        }

        return true;
    }

    /// returns true if some cell in the board has no candidates
    pub fn has_contradiction(&self) -> bool {
        return self.cells.iter().any(|cell| cell.is_empty());
    }

    /// returns true if every cell in the board has been solved (exactly 1 candidate)
    pub fn is_solved(&self) -> bool {
        return self.solved_count == 81;
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
                        let digit = ((candidate_row * 3) + candidate_col + 1) as u8;
                        let is_digit_candidate = cell.contains(digit);

                        if is_digit_candidate {
                            write!(out, "{}", digit).unwrap();
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
                    write!(out, "{}", digit).unwrap();
                } else {
                    write!(out, " ").unwrap();
                }
            }

            writeln!(out).unwrap();
        }

        return out;
    }
}

// designed to support parsing from strings in https://github.com/grantm/sudoku-exchange-puzzle-bank
impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();

        let mut position_id = 0;
        for character in s.chars() {
            let digit_val = match character {
                '1'..='9' => character as u8 - b'0',
                '.' | '?' | '0' | ' ' => {
                    position_id += 1;
                    continue;
                }
                '\n' | '\r' | '\t' | '/' => continue,
                _ => return Err(format!("Encountered unexpected character: {}", character)),
            };

            let position = Position::from_id(position_id);
            if !board.cells[position.id() as usize].is_solved() {
                board.solve_cell(position, digit_val);
            }
            position_id += 1;
        }

        return Ok(board);
    }
}
