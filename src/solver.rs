use crate::board::{board::Board, position::{MAX_POSITION_ID, Position}};

/// uses backtracing with MRV heuristic to solve the puzzle
/// ref: https://www.geeksforgeeks.org/artificial-intelligence/explain-the-concept-of-backtracking-search-and-its-role-in-finding-solutions-to-csps/
/// returns Some(solution) if a solution is found, and None if no solution is found
pub fn solve(board: &Board) -> Option<Board> {
    if board.is_solved() {
        return Some(board.to_owned());
    }

    let (mut fewest_candidates_position_id, mut fewest_candidates_count) = (0, 10u32);

    // check every position on the board, find the position with the fewest candidates
    for position_id in 0..=MAX_POSITION_ID {
        let cell = board.at(Position::from_id(position_id));

        // if this position has no candidates, the board is now invalid
        let count = cell.candidates_count();
        if count == 0 {
            return None;
        }

        // if this cell is not solved and found cell with fewer candidates, pick that one instead
        if !cell.is_solved() && count < fewest_candidates_count {
            (fewest_candidates_position_id, fewest_candidates_count) = (position_id, count);

            // if we can immediately solve this cell, stop searching around and solve it
            if count == 2 {
                break;
            }
        }
    }

    // if NO cells have fewer than 10 candidates, there is obviously no solution
    // but also this should really never happen
    if fewest_candidates_count == 10u32 {
        eprintln!("Something went wrong: no cells have fewer than 10 candidates");
        return None;
    }

    let best_cell_position = Position::from_id(fewest_candidates_position_id);
    let best_cell = board.at(best_cell_position);

    for digit in best_cell.iter() {
        let mut attempt = board.to_owned();

        // if we can solve this cell
        if attempt.solve_cell(best_cell_position, digit) {
            // and it led to a solution, solved the board
            if let Some(solution) = solve(&attempt) {
                return Some(solution);
            }
        }
    }

    return None;
}
