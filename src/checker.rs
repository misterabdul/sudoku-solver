pub enum Status {
    Wrong,
    Unsolved,
    Solved,
}

const EVERY_NUMBER_COUNT: usize = 0x3fe; // 0b1111111110

pub fn check(puzzle: &[[u8; 9]; 9]) -> Status {
    let mut result = Status::Solved;

    for i in 0..9 {
        match check_row(puzzle, i) {
            Status::Wrong => return Status::Wrong,
            Status::Unsolved => result = Status::Unsolved,
            Status::Solved => {}
        }
        match check_col(puzzle, i) {
            Status::Wrong => return Status::Wrong,
            Status::Unsolved => result = Status::Unsolved,
            Status::Solved => {}
        }
        match match i {
            0 => check_sub(puzzle, 0, 0),
            1 => check_sub(puzzle, 0, 3),
            2 => check_sub(puzzle, 0, 6),
            3 => check_sub(puzzle, 3, 0),
            4 => check_sub(puzzle, 3, 3),
            5 => check_sub(puzzle, 3, 6),
            6 => check_sub(puzzle, 6, 0),
            7 => check_sub(puzzle, 6, 3),
            8 => check_sub(puzzle, 6, 6),
            _ => Status::Wrong,
        } {
            Status::Wrong => return Status::Wrong,
            Status::Unsolved => result = Status::Unsolved,
            Status::Solved => {}
        }
    }

    return result;
}

pub fn check_row(puzzle: &[[u8; 9]; 9], row: usize) -> Status {
    let mut counts: usize = 0;

    for j in 0..9 {
        if puzzle[row][j] > 0 && (counts & (0x1 << puzzle[row][j])) > 0 {
            return Status::Wrong;
        }
        counts |= 0x1 << puzzle[row][j];
    }

    return if counts == EVERY_NUMBER_COUNT {
        Status::Solved
    } else {
        Status::Unsolved
    };
}

pub fn check_col(puzzle: &[[u8; 9]; 9], col: usize) -> Status {
    let mut counts: usize = 0;

    for i in 0..9 {
        if puzzle[i][col] > 0 && (counts & (0x1 << puzzle[i][col])) > 0 {
            return Status::Wrong;
        }
        counts |= 0x1 << puzzle[i][col];
    }

    return if counts == EVERY_NUMBER_COUNT {
        Status::Solved
    } else {
        Status::Unsolved
    };
}

pub fn check_sub(puzzle: &[[u8; 9]; 9], mut row: usize, mut col: usize) -> Status {
    let mut counts: usize = 0;

    row = (row / 3) * 3;
    col = (col / 3) * 3;
    for i in row..row + 3 {
        for j in col..col + 3 {
            if puzzle[i][j] > 0 && counts & (0x1 << puzzle[i][j]) > 0 {
                return Status::Wrong;
            }
            counts |= 0x1 << puzzle[i][j];
        }
    }

    return if counts == EVERY_NUMBER_COUNT {
        Status::Solved
    } else {
        Status::Unsolved
    };
}
