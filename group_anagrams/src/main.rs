// given an array of strings, group anagrams together. You can return the answer in any order.
fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();
    chars1.sort_unstable();
    chars2.sort_unstable();
    chars1 == chars2
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: Vec<Vec<String>> = Vec::new();
    for s in strs {
        let mut found = false;
        for group in &mut anagrams {
            let s_clone = s.clone();
            if is_anagram(&s_clone, &group[0]) {
                group.push(s_clone);
                found = true;
                break;
            }
        }
        if !found {
            anagrams.push(vec![s]);
        }
    }
    anagrams
}

//need fn main
fn main() {
    let strs = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];
    let result = group_anagrams(strs);
    println!("{:?}", result);
}

// testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let strs = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let result = group_anagrams(strs);
        assert_eq!(result.len(), 3);
    }
    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("eat", "tea"));
        assert!(!is_anagram("eat", "tan"));
    }
}