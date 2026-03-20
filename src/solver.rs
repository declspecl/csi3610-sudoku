use crate::board::{board::Board, position::{MAX_POSITION_ID, Position, PositionId}};

/// uses backtracing with MRV heuristic to solve the puzzle
/// ref: https://www.geeksforgeeks.org/artificial-intelligence/explain-the-concept-of-backtracking-search-and-its-role-in-finding-solutions-to-csps/
pub fn solve(board: &Board) -> Option<Board> {
    if board.has_contradiction() {
        return None;
    } else if board.is_solved() {
        return Some(board.to_owned());
    }

    let (mut fewest_candidates_position_id, mut fewest_candidates_count) = (
        0 as PositionId,
        board.at(Position::from_id(0)).candidates_count()
    );

    for position_id in 1..=MAX_POSITION_ID {
        let cell = board.at(Position::from_id(position_id));
        let count = cell.candidates_count();

        if count > 1 && count < fewest_candidates_count {
            (fewest_candidates_position_id, fewest_candidates_count) = (position_id, count);
        }
    }

    let fewest_candidates_position = Position::from_id(fewest_candidates_position_id);
    let fewest_candidates_digits = board.at(fewest_candidates_position);

    for digit in fewest_candidates_digits.iter() {
        let mut attempt = board.to_owned();

        if attempt.solve_cell(fewest_candidates_position, digit) {
            if let Some(solution) = solve(&attempt) {
                return Some(solution);
            }
        }
    }

    return None;
}
