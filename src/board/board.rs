use crate::board::{digit::Digit, digit_candidate_set::DigitCandidateSet, position::Position};

pub const BOARD_LENGTH: u8 = 9;
pub const BOARD_CELL_COUNT: u8 = BOARD_LENGTH * BOARD_LENGTH;

#[derive(Debug)]
pub struct Board {
    cells: [DigitCandidateSet; BOARD_CELL_COUNT as usize]
}

impl Board {
    const EMPTY_CELLS: [DigitCandidateSet; BOARD_CELL_COUNT as usize] = [DigitCandidateSet::ALL; BOARD_CELL_COUNT as usize];

    pub fn new() -> Self {
        return Self { cells: Self::EMPTY_CELLS };
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
}

