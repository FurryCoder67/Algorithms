use std::collections::HashMap;

fn frequent_elements(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut counts = HashMap::new();
    for num in nums {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }
    // Turn the HashMap into a list of (number, count) pairs.
    let mut elements: Vec<(i32, i32)> = counts.into_iter().collect();
    // Sort by count, from highest to lowest.
    elements.sort_by(|a, b| b.1.cmp(&a.1));
    // Take the first k numbers.
    let mut result = Vec::new();

    for i in 0..k.min(elements.len()) {
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