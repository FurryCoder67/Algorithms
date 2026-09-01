use std::collections::HashMap;

fn frequent_elements(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut counts = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mut elements: Vec<(i32, i32)> = counts.into_iter().collect();
    let n = elements.len();
    let k = k.min(n);

    for i in 0..k {
        for j in (i + 1..n).rev() {
            if elements[j].1 > elements[j - 1].1 {
                elements.swap(j, j - 1);
            }
        }
    }

    elements.truncate(k);
    elements.shrink_to_fit();

    elements.into_iter().map(|(value, _)| value).collect()
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = frequent_elements(nums, k);
    println!("{:?}", result);
}
