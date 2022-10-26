use std::vec::Vec;

use crate::checker;

pub fn collect_possibilities(mut puzzle: [[u8; 9]; 9]) -> [[Vec<u8>; 9]; 9] {
    const EMPTY_LINKED_LIST: Vec<u8> = Vec::new();
    const EMPTY_LINKED_LIST_ARRAY: [Vec<u8>; 9] = [EMPTY_LINKED_LIST; 9];

    let mut result = [EMPTY_LINKED_LIST_ARRAY; 9];

    for i in 0..9 {
        for j in 0..9 {
            if puzzle[i][j] > 0 {
                result[i][j].push(puzzle[i][j])
            } else {
                for k in 0..9 {
                    puzzle[i][j] = k + 1;
                    if match checker::check_row(&puzzle, i) {
                        checker::Status::Wrong => false,
                        _ => true,
                    } && match checker::check_col(&puzzle, j) {
                        checker::Status::Wrong => false,
                        _ => true,
                    } && match checker::check_sub(&puzzle, i, j) {
                        checker::Status::Wrong => false,
                        _ => true,
                    } {
                        result[i][j].push(k + 1)
                    }
                }
                puzzle[i][j] = 0;
            }
        }
    }

    return result;
}
