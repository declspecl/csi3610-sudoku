#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digit(u8);

impl Digit {
    pub const ONE: Self = Digit(1);
    pub const TWO: Self = Digit(2);
    pub const THREE: Self = Digit(3);
    pub const FOUR: Self = Digit(4);
    pub const FIVE: Self = Digit(5);
    pub const SIX: Self = Digit(6);
    pub const SEVEN: Self = Digit(7);
    pub const EIGHT: Self = Digit(8);
    pub const NINE: Self = Digit(9);

    pub const ALL: [Self; 9] = [
        Self::ONE,
        Self::TWO,
        Self::THREE,
        Self::FOUR,
        Self::FIVE,
        Self::SIX,
        Self::SEVEN,
        Self::EIGHT,
        Self::NINE,
    ];

    pub const fn as_u8(self) -> u8 {
        return self.0;
    }
}

impl From<Digit> for u8 {
    fn from(value: Digit) -> Self {
        return value.as_u8();
    }
}

