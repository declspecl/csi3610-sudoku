use crate::board::{board::Board, digit::Digit, digit_candidate_set::DigitCandidateSet, position::Position};

pub mod board;

fn main() {
    let board = Board::new();
    println!("Board: {:?}", board);

    let mut top_left = board.at(Position::new(Digit::ONE, Digit::ONE));
    top_left = top_left.remove(Digit::ONE);
    top_left = top_left.remove(Digit::TWO);
    top_left = top_left.remove(Digit::THREE);
    top_left = top_left.remove(Digit::FOUR);
    top_left = top_left.remove(Digit::FIVE);
    top_left = top_left.remove(Digit::SIX);
    top_left = top_left.remove(Digit::SEVEN);
    top_left = top_left.remove(Digit::EIGHT);
    top_left = top_left.remove(Digit::NINE);

    println!("Testing {:?} = {:?}", top_left, DigitCandidateSet::NONE);
    assert_eq!(top_left, DigitCandidateSet::NONE);
}
