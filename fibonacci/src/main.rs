fn fibonacci_sequence(start: [i64; 2], length: usize) -> Vec<i64> {
    if length == 0 {
        return Vec::new();
    }

    if length == 1 {
        return vec![start[0]];
    }

    let mut sequence = vec![start[0], start[1]];

    while sequence.len() < length {
        let n = sequence.len();
        let next = sequence[n - 2] + sequence[n - 1];
        sequence.push(next);
    }

    sequence
}

fn main() {
    assert_eq!(
        fibonacci_sequence([0, 1], 20),
        vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34,
            55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181
        ]
    );

    assert_eq!(
        fibonacci_sequence([21, 32], 1),
        vec![21]
    );

    assert_eq!(
        fibonacci_sequence([0, 1], 0),
        vec![]
    );

    assert_eq!(
        fibonacci_sequence([10, 20], 2),
        vec![10, 20]
    );

    assert_eq!(
        fibonacci_sequence([123456789, 987654321], 5),
        vec![123456789, 987654321, 1111111110, 2098765431, 3209876541]
    );

    println!("#pig_emoji!");
}