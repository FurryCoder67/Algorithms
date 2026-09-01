use std::collections::HashMap;

fn frequent_elements(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut counts = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
        if counts.len() > k + 1 {
            let min_key = map.iter().min_by_key(|(_, c)| **v).map(|(_, c)| *k)
            if let Some(key) = min_key {
                map.remove(&key);
            }
        }
    }
    let mut elements: Vec<(i32, i32)> = counts.into_iter().collect();
    elements.sort_by(|a, b| b.1.cmp(&a.1));
    let mut result = Vec::new();
    for i in 0..k {
        result.push(elements[i].0);
    }
    result
}
fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = frequent_elements(nums, k);
    println!("{:?}", result);
}