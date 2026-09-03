use std::collections::HashSet;

struct SudokuBoard {
    board: Vec<Vec<char>>,
}

impl SudokuBoard {
    fn new(board: Vec<Vec<char>>) -> Self {
        Self { board }
    }

    fn is_valid(&self) -> bool {
        self.are_rows_valid()
            && self.are_columns_valid()
            && self.are_boxes_valid()
    }

    fn are_rows_valid(&self) -> bool {
        for row in 0..9 {
            if !self.is_group_valid(
                (0..9).map(|col| self.board[row][col])
            ) {
                return false;
            }
        }

        true
    }

    fn are_columns_valid(&self) -> bool {
        for col in 0..9 {
            if !self.is_group_valid(
                (0..9).map(|row| self.board[row][col])
            ) {
                return false;
            }
        }

        true
    }

    fn are_boxes_valid(&self) -> bool {
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                let values = (box_row..box_row + 3)
                    .flat_map(|row| {
                        (box_col..box_col + 3)
                            .map(move |col| self.board[row][col])
                    });

                if !self.is_group_valid(values) {
                    return false;
                }
            }
        }

        true
    }

    fn is_group_valid<I>(&self, values: I) -> bool
    where
        I: IntoIterator<Item = char>,
    {
        let mut seen = HashSet::new();

        for value in values {
            if value != '.' && !seen.insert(value) {
                return false;
            }
        }

        true
    }
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
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let valid_sudoku = SudokuBoard::new(valid_board);
    let invalid_sudoku = SudokuBoard::new(invalid_board);

    assert!(valid_sudoku.is_valid());
    assert!(!invalid_sudoku.is_valid());

    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_board(rows: [&str; 9]) -> SudokuBoard {
        let board = rows
            .iter()
            .map(|row| row.chars().collect())
            .collect();

        SudokuBoard::new(board)
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

        assert!(board.is_valid());
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

        assert!(board.is_valid());
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

        assert!(!board.is_valid());
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

        assert!(!board.is_valid());
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

        assert!(!board.is_valid());
    }
}