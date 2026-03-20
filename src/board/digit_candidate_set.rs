use crate::board::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitCandidateSet(u16);

/// wrapper type to avoid Box<dyn Iterator<Item = Digit>>
pub struct DigitCandidateIter(u16);

impl Iterator for DigitCandidateIter {
    type Item = Digit;

    // ref: https://stackoverflow.com/questions/67990260/how-to-iterate-over-subsets-of-a-bitwise-mask
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        // 3 trailing zero means next lowest digit is 3
        let bit = self.0.trailing_zeros();

        // clears all the low bits up to and including the bit at the position `bit` above
        // effectively sets the bitmask such that the new lowest bit is the next lowest digit in the set
        self.0 &= self.0 - 1;

        return Some(Digit::ALL[(bit - 1) as usize]);
    }
}

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

    pub fn iter(self) -> DigitCandidateIter {
        return DigitCandidateIter(self.0);
    }

    pub fn solved_digit(self) -> Option<Digit> {
        if self.is_solved() {
            return Some(Digit::from(match self.0.trailing_zeros() {
                1 => Digit::ONE,
                2 => Digit::TWO,
                3 => Digit::THREE,
                4 => Digit::FOUR,
                5 => Digit::FIVE,
                6 => Digit::SIX,
                7 => Digit::SEVEN,
                8 => Digit::EIGHT,
                9 => Digit::NINE,
                _ => unreachable!(),
            }));
        }

        return None;
    }
}

