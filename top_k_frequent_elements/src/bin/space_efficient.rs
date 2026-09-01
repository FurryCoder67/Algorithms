use std::collections::HashMap;

fn frequent_elements(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut counts = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
        // assume that the vector you are given is sorted. add if statements after line 6 to remove the unnessecary values in the hashmap that arent going to be in the end for sure because k or more other values have more than that value
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