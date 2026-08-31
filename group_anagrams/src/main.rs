// given an array of strings, group anagrams together. You can return the answer in any order.


fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: Vec<Vec<String>> = Vec::new();
    for s in strs {
        let mut found = false;
        for group in &mut anagrams {
            // scope doesnt work with &s, so we need to clone it
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