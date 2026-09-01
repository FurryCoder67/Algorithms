use std::collections::HashMap;

fn frequent_elements(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    let len = nums.len();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Bucket by frequency (1..=len) instead of sorting: a frequency can
    // never exceed the number of elements, so this is O(n) instead of
    // O(m log m) for m distinct values.
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); len + 1];
    for (num, count) in counts {
        buckets[count].push(num);
    }

    let mut result = Vec::with_capacity(k);
    for bucket in buckets.into_iter().rev() {
        for num in bucket {
            if result.len() == k {
                return result;
            }
            result.push(num);
        }
    }
    result
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = frequent_elements(nums, k);
    println!("{:?}", result);
}
