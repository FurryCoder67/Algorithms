use std::collections::HashSet;

fn valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for row in 0..9 {
        let mut seen = HashSet::new();

        for col in 0..9 {
            let value = board[row][col];

            if value != '.' && !seen.insert(value) {
                return false;
            }
        }
    }

    for col in 0..9 {
        let mut seen = HashSet::new();

        for row in 0..9 {
            let value = board[row][col];

            if value != '.' && !seen.insert(value) {
                return false;
            }
        }
    }

    for box_row in (0..9).step_by(3) {
        for box_col in (0..9).step_by(3) {
            let mut seen = HashSet::new();

            for row in box_row..box_row + 3 {
                for col in box_col..box_col + 3 {
                    let value = board[row][col];

                    if value != '.' && !seen.insert(value) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

