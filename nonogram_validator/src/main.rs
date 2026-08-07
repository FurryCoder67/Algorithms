fn is_valid_nonogram(clue: &[i32], cells: &[i32]) -> bool {
    let mut groups = Vec::new();
    let mut count = 0;
    for &cell in cells {
        if cell == 1 {
            count += 1;
        } else if count > 0 {
            groups.push(count);
            count = 0;
        }
    }
    if count > 0 {
        groups.push(count);
    }
    groups == clue
}

fn main() {
    println!(
        "{}",
        is_valid_nonogram(&[3, 2], &[1, 1, 1, 0, 1, 1])
    ); // true

    println!(
        "{}",
        is_valid_nonogram(&[3, 2], &[0, 1, 1, 1, 1, 1])
    ); // false

    println!(
        "{}",
        is_valid_nonogram(&[1, 1, 1, 1], &[1, 0, 1, 0, 1, 0, 1, 0, 1])
    ); // false

    println!(
        "{}",
        is_valid_nonogram(
            &[1, 1, 1, 1],
            &[0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]
        )
    ); // true

    println!(
        "{}",
        is_valid_nonogram(
            &[3, 2, 3],
            &[0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0]
        )
    ); // true

    println!(
        "{}",
        is_valid_nonogram(
            &[3, 2, 3],
            &[0, 0, 0, 1, 0, 0, 1, 0, 0, 0]
        )
    ); // false
}