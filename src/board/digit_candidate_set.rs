use crate::board::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitCandidateSet(u16);

impl DigitCandidateSet {
    pub const ALL: Self = Self(0b_0000_0011_1111_1110);
    pub const NONE: Self = Self(0);

    pub fn of(digit: Digit) -> Self {
        return Self(1 << digit.as_u8());
    }

    pub fn contains(self, digit: Digit) -> bool {
        return (self.0 >> digit.as_u8()) & 0b1 == 0b1
    }

    pub fn remove(self, digit: Digit) -> Self {
        return Self(self.0 & !(1 << digit.as_u8()));
    }

    pub fn candidates_count(self) -> u32 {
        return self.0.count_ones();
    }

    pub fn is_empty(self) -> bool {
        return self.0 == 0;
    }
}

