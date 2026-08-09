#[derive(Debug)]
struct Bucket {
    color: [i32; 3],
    fullness: i32,
}

fn mix_paint(a: Bucket, b: Bucket) -> [i32; 3] {
    let total_fullness = a.fullness + b.fullness;
    [
        ((a.color[0] * a.fullness + b.color[0] * b.fullness) as f64 / total_fullness as f64).round() as i32,
        ((a.color[1] * a.fullness + b.color[1] * b.fullness) as f64 / total_fullness as f64).round() as i32,
        ((a.color[2] * a.fullness + b.color[2] * b.fullness) as f64 / total_fullness as f64).round() as i32,
    ]
}

fn main() {
    assert_eq!(
        mix_paint(
            Bucket { color: [250, 250, 250], fullness: 50 },
            Bucket { color: [0, 0, 0], fullness: 50 },
        ),
        [125, 125, 125]
    );

    assert_eq!(
        mix_paint(
            Bucket { color: [250, 250, 250], fullness: 80 },
            Bucket { color: [0, 0, 0], fullness: 20 },
        ),
        [200, 200, 200]
    );

    assert_eq!(
        mix_paint(
            Bucket { color: [100, 150, 200], fullness: 30 },
            Bucket { color: [100, 150, 200], fullness: 70 },
        ),
        [100, 150, 200]
    );

    assert_eq!(
        mix_paint(
            Bucket { color: [143, 143, 101], fullness: 45 },
            Bucket { color: [100, 204, 204], fullness: 90 },
        ),
        [114, 184, 170]
    );

    assert_eq!(
        mix_paint(
            Bucket { color: [15, 134, 249], fullness: 29 },
            Bucket { color: [97, 178, 55], fullness: 54 },
        ),
        [68, 163, 123]
    );

    println!("All tests passed!");
}