use crate::board::{digit_candidate_set::DigitCandidateSet, position::Position};

const BOARD_LENGTH: u8 = 9;
const BOARD_CELL_COUNT: u8 = BOARD_LENGTH * BOARD_LENGTH;

#[derive(Debug)]
pub struct Board {
    cells: [DigitCandidateSet; BOARD_CELL_COUNT as usize]
}

impl Board {
    pub fn new() -> Self {
        return Self { cells: [DigitCandidateSet::ALL; BOARD_CELL_COUNT as usize] };
    }

    pub fn at(&self, position: Position) -> DigitCandidateSet {
        let row_offset = (BOARD_LENGTH * (position.row.as_u8() - 1)) as usize;
        let col_offset = (position.col.as_u8() - 1) as usize;

        return self.cells.get(row_offset + col_offset)
            .unwrap()
            .to_owned();
    }
}

