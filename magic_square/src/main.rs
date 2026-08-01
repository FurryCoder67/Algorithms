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
        if is_magic(&grid) {
            return "valid".to_string();
        } else {
            return "impossible".to_string();
        }
    }
    for candidate in -1000..=1000 {
        if candidate == 0 {
            continue;
        }

        grid[zero_row][zero_col] = candidate;

        if is_magic(&grid) {
            return candidate.to_string();
        }
    }

    "impossible".to_string()
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
        println!(
            "Test {}: {}",
            i + 1,
            solve_magic_square(*test)
        );
    }
}