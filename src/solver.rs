use crate::board::{board::Board, position::{MAX_POSITION_ID, Position}};

/// uses backtracing with MRV heuristic to solve the puzzle
/// ref: https://www.geeksforgeeks.org/artificial-intelligence/explain-the-concept-of-backtracking-search-and-its-role-in-finding-solutions-to-csps/
pub fn solve(board: &Board) -> Option<Board> {
    if board.has_contradiction() {
        return None;
    } else if board.is_solved() {
        return Some(board.to_owned());
    }

    let (mut fewest_candidates_position_id, mut fewest_candidates_count) = (0, u32::MAX);

    for position_id in 0..=MAX_POSITION_ID {
        let count = board.at(Position::from_id(position_id)).candidates_count();

        if count > 1 && count < fewest_candidates_count {
            (fewest_candidates_position_id, fewest_candidates_count) = (position_id, count);
        }
    }

    if fewest_candidates_count == u32::MAX {
        panic!("The board isn't solved and isn't contradictory. There should exist at least one unsolved cell (candidates_count > 1).");
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
