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

fn main() {
    let valid_board = vec![
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

    let invalid_board = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '8'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(valid_sudoku(valid_board));
    assert!(!valid_sudoku(invalid_board));
    println!("All tests passed!")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_board(rows: [&str; 9]) -> Vec<Vec<char>> {
        rows.iter()
            .map(|r| r.chars().collect())
            .collect()
    }

    #[test]
    fn test_valid_board() {
        let board = create_board([
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(valid_sudoku(board));
    }

    #[test]
    fn test_empty_board() {
        let board = create_board([
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
        ]);
        assert!(valid_sudoku(board));
    }

    #[test]
    fn test_duplicate_in_row() {
        let board = create_board([
            "53..7...5",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!valid_sudoku(board));
    }

    #[test]
    fn test_duplicate_in_column() {
        let board = create_board([
            "53..7....",
            "6..195...",
            "598....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!valid_sudoku(board));
    }

    #[test]
    fn test_duplicate_in_3x3_subgrid() {
        let board = create_board([
            "53..7....",
            "6..195...",
            ".58....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!valid_sudoku(board));
    }
}