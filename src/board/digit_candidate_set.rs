use crate::board::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitCandidateSet(u16);

impl DigitCandidateSet {
    pub const ALL: Self = Self(0b_0000_0011_1111_1110);
    pub const NONE: Self = Self(0);

    pub const fn of(digit: Digit) -> Self {
        return Self(1 << digit.as_u8());
    }

    pub const fn contains(self, digit: Digit) -> bool {
        return (self.0 >> digit.as_u8()) & 0b1 == 0b1
    }

    pub const fn add(self, digit: Digit) -> Self {
        return Self(self.0 | (1 << digit.as_u8()));
    }

    pub const fn remove(self, digit: Digit) -> Self {
        return Self(self.0 & !(1 << digit.as_u8()));
    }

    pub const fn candidates_count(self) -> u32 {
        return self.0.count_ones();
    }

    pub const fn is_empty(self) -> bool {
        return self.0 == 0;
    }

    pub const fn is_solved(self) -> bool {
        return self.candidates_count() == 1;
    }
}

