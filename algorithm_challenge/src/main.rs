fn palindrome(s: String) -> i32 {
    // Make this algorithm return a number that is the amount of palindromic substrings
    let mut count = 0;
    let n = s.len();
    for i in 0..n {
        for j in (i + 1)..=n {
            let substring = &s[i..j];
            let reversed = substring.chars().rev().collect::<String>();
            if substring == reversed {
                count += 1;
            }
        }
    }
    count
}
fn main() {
    println!("{}", palindrome("racecar".into()));
    println!("{}", palindrome("poop".into()));
    println!("{}", palindrome("something".into()));
}