// given 2 strings, return true if t is an anagram of s and false otherwise

fn valid_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort_unstable();
    t_chars.sort_unstable();
    s_chars == t_chars
}
// testing
fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    println!("{}", valid_anagram(s, t));
}

