use crate::board::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: Digit,
    pub col: Digit
}

impl Position {
    pub fn new(row: Digit, col: Digit) -> Self {
        return Self { row, col };
    }
}

