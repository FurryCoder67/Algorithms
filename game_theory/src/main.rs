fn play_game(player1: &str, player2: &str) -> [i32; 2] {
    let mut score1 = 0;
    let mut score2 = 0;

    for (p1, p2) in player1.chars().zip(player2.chars()) {
        match (p1, p2) {
            ('C', 'C') => {
                score1 += 3;
                score2 += 3;
            }
            ('D', 'D') => {
                score1 += 1;
                score2 += 1;
            }
            ('D', 'C') => {
                score1 += 5;
            }
            ('C', 'D') => {
                score2 += 5;
            }
            _ => {  }
        }
    }

    [score1, score2]
}

fn main() {
    let tests = [
        ("CCCC", "CCCC", [12, 12]),
        ("DDDD", "DDDD", [4, 4]),
        ("CCDD", "CDDD", [5, 10]),
        ("CCCDCDCCCDDC", "CCDDCDCDDCCD", [24, 34]),
        (
            "DDCCDDDDCDDCDDDCDD",
            "CCDCCCDCCCDCCCCDCC",
            [66, 21],
        ),
    ];

    for (i, (p1, p2, expected)) in tests.iter().enumerate() {
        let result = play_game(p1, p2);
        if result == *expected {
            println!("Test {}: PASS {:?}", i + 1, result);
        } else {
            println!(
                "Test {}: FAIL\n  Expected: {:?}\n  Got:      {:?}",
                i + 1,
                expected,
                result
            );
        }
    }
}