use crate::checker;

pub fn brute_force(
    mut puzzle: [[u8; 9]; 9],
    possibilities: [[Vec<u8>; 9]; 9],
) -> Result<[[u8; 9]; 9], Error> {
    return match do_brute_force(&mut puzzle, &possibilities, 0, 0) {
        Ok(()) => Ok(puzzle),
        Err(error) => Err(error),
    };
}

fn do_brute_force(
    puzzle: &mut [[u8; 9]; 9],
    possibilities: &[[Vec<u8>; 9]; 9],
    row: usize,
    col: usize,
) -> Result<(), Error> {
    if row > 8 {
        // one last check just to make sure
        return match checker::check(puzzle) {
            checker::Status::Solved => Ok(()),
            _ => Err(Error {}),
        };
    }

    let mut next_row = row;
    let mut next_col = col + 1;
    if next_col > 8 {
        next_col = 0;
        next_row += 1;
    }
    if puzzle[row][col] <= 0 {
        for val in &possibilities[row][col] {
            puzzle[row][col] = *val;
            if match checker::check_row(puzzle, row) {
                checker::Status::Wrong => false,
                _ => true,
            } && match checker::check_col(puzzle, col) {
                checker::Status::Wrong => false,
                _ => true,
            } && match checker::check_sub(puzzle, row, col) {
                checker::Status::Wrong => false,
                _ => true,
            } {
                match do_brute_force(puzzle, possibilities, next_row, next_col) {
                    Ok(()) => return Ok(()),
                    _ => {}
                }
            }
        }
        puzzle[row][col] = 0;
    } else {
        return do_brute_force(puzzle, possibilities, next_row, next_col);
    }

    return Err(Error {});
}

pub struct Error {}
