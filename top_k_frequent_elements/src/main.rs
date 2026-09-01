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

// Quickselect: partition unique (value, count) pairs around a pivot count,
// like "kth largest element," recursing only into the side holding the
// cutoff rank. Average O(n) with no auxiliary bucket array, but worst-case
// O(n^2) on an unlucky pivot sequence, so the pivot is chosen randomly.
fn frequent_elements_quickselect(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mut pairs: Vec<(i32, i32)> = counts.into_iter().collect();
    let n = pairs.len();
    if k == 0 || n == 0 {
        return Vec::new();
    }
    if k >= n {
        return pairs.into_iter().map(|(num, _)| num).collect();
    }

    let mut rng_state: u32 = 0x9E3779B9 ^ (n as u32);
    let mut next_rand = |bound: usize| -> usize {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 17;
        rng_state ^= rng_state << 5;
        (rng_state as usize) % bound
    };

    let (mut left, mut right) = (0usize, n - 1);
    loop {
        let pivot_index = left + next_rand(right - left + 1);
        let pivot_index = partition_by_count(&mut pairs, left, right, pivot_index);
        if pivot_index == k - 1 {
            break;
        } else if pivot_index < k - 1 {
            left = pivot_index + 1;
        } else {
            right = pivot_index - 1;
        }
    }

    pairs.into_iter().take(k).map(|(num, _)| num).collect()
}

// Lomuto partition ordering by descending count, so index 0 ends up holding
// the highest frequency.
fn partition_by_count(pairs: &mut [(i32, i32)], left: usize, right: usize, pivot_index: usize) -> usize {
    let pivot_count = pairs[pivot_index].1;
    pairs.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..right {
        if pairs[i].1 > pivot_count {
            pairs.swap(store_index, i);
            store_index += 1;
        }
    }
    pairs.swap(store_index, right);
    store_index
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    println!("bucket sort:  {:?}", frequent_elements(nums.clone(), k));
    println!("quickselect:  {:?}", frequent_elements_quickselect(nums, k));
}
