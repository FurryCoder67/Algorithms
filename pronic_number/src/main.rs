fn is_pronic(n: i32) -> bool {
    if n < 0 {
        return false;
    }
    let mut i = 0;
    while i * (i + 1) <= n {
        if i * (i + 1) == n {
            return true;
        }
        i += 1;
    }
    false
}

fn main() {
    println!("isPronic(6) = {} (expected: true)", is_pronic(6));
    println!("isPronic(15) = {} (expected: false)", is_pronic(15));
    println!("isPronic(12) = {} (expected: true)", is_pronic(12));
    println!("isPronic(132) = {} (expected: true)", is_pronic(132));
    println!("isPronic(80) = {} (expected: false)", is_pronic(80));
    println!("isPronic(0) = {} (expected: true)", is_pronic(0));
}