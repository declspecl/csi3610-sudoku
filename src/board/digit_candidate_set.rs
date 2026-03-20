#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitCandidateSet(u16);

/// wrapper type to avoid Box<dyn Iterator<Item = u8>>
pub struct DigitCandidateIter(u16);

impl Iterator for DigitCandidateIter {
    type Item = u8;

    // ref: https://stackoverflow.com/questions/67990260/how-to-iterate-over-subsets-of-a-bitwise-mask
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let bit = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;

        return Some(bit);
    }
}

impl DigitCandidateSet {
    pub const ALL: Self = Self(0b_0000_0011_1111_1110);
    pub const NONE: Self = Self(0);

    pub const fn of(digit: u8) -> Self {
        return Self(1 << digit);
    }

    pub const fn contains(self, digit: u8) -> bool {
        return (self.0 >> digit) & 1 == 1;
    }

    pub const fn add(self, digit: u8) -> Self {
        return Self(self.0 | (1 << digit));
    }

    pub const fn remove(self, digit: u8) -> Self {
        return Self(self.0 & !(1 << digit));
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

    pub fn iter(self) -> DigitCandidateIter {
        return DigitCandidateIter(self.0);
    }

    pub const fn solved_digit(self) -> Option<u8> {
        if self.is_solved() {
            return Some(self.0.trailing_zeros() as u8);
        }

        return None;
    }
}
