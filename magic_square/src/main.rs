fn is_magic(grid: &[[i32; 3]; 3]) -> bool {
    let target = grid[0][0] + grid[0][1] + grid[0][2];

    for row in 0..3 {
        let sum = grid[row][0] + grid[row][1] + grid[row][2];
        if sum != target {
            return false;
        }
    }

    for col in 0..3 {
        let sum = grid[0][col] + grid[1][col] + grid[2][col];
        if sum != target {
            return false;
        }
    }

    let diag1 = grid[0][0] + grid[1][1] + grid[2][2];
    let diag2 = grid[0][2] + grid[1][1] + grid[2][0];

    diag1 == target && diag2 == target
}

fn solve_magic_square(mut grid: [[i32; 3]; 3]) -> String {
    let mut zero_row = 0;
    let mut zero_col = 0;
    let mut found = false;

    for r in 0..3 {
        for c in 0..3 {
            if grid[r][c] == 0 {
                zero_row = r;
                zero_col = c;
                found = true;
            }
        }
    }

    if !found {
        return if is_magic(&grid) {
            "valid".to_string()
        } else {
            "impossible".to_string()
        };
    }

    let mut target = None;

    for r in 0..3 {
        if r != zero_row {
            target = Some(grid[r][0] + grid[r][1] + grid[r][2]);
            break;
        }
    }

    let target = target.unwrap();

    let current_sum =
        grid[zero_row][0] + grid[zero_row][1] + grid[zero_row][2];

    let missing = target - current_sum;

    grid[zero_row][zero_col] = missing;

    if is_magic(&grid) {
        missing.to_string()
    } else {
        "impossible".to_string()
    }
}

fn main() {
    let tests = vec![
        [[2, 7, 6], [9, 0, 1], [4, 3, 8]],
        [[0, 14, 12], [18, 10, 2], [8, 6, 16]],
        [[12, 17, 16], [19, 0, 10], [14, 13, 18]],
        [[15, 35, 31], [43, 27, 11], [23, 19, 0]],
        [[26, 41, 14], [47, 35, 0], [32, 29, 44]],
    ];

    for (i, test) in tests.iter().enumerate() {
        println!("Test {}: {}", i + 1, solve_magic_square(*test));
    }
}