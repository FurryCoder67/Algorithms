use std::collections::HashSet;

fn valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // Check rows
    for row in 0..9 {
        let mut seen = HashSet::new();

        for col in 0..9 {
            let value = board[row][col];

            if value != '.' && !seen.insert(value) {
                return false;
            }
        }
    }

    // Check columns
    for col in 0..9 {
        let mut seen = HashSet::new();

        for row in 0..9 {
            let value = board[row][col];

            if value != '.' && !seen.insert(value) {
                return false;
            }
        }
    }

    // Check 3x3 boxes
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

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("Valid Sudoku: {}", valid_sudoku(board));
}