// given an array of strings, group anagrams together. You can return the answer in any order.

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();

        map.entry(key).or_insert(Vec::new()).push(s);
    }

    map.into_values().collect()
}

// testing
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