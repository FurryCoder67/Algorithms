// No HashMap: extra space is O(k) (the `top` buffer) instead of O(distinct values).
// Traded away for time: sorting in place is O(n^2) instead of O(n) hashing.
fn frequent_elements(mut nums: Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();
    let k = k.min(n);

    // In-place bubble sort so equal values end up adjacent, with no second buffer.
    for i in 0..n {
        for j in 0..n - 1 - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }

    // Only the k best (value, count) pairs seen so far are kept, sorted ascending by count
    // so the current cutoff to beat is always at index 0.
    let mut top: Vec<(i32, i32)> = Vec::with_capacity(k);

    let mut i = 0;
    while i < n {
        let value = nums[i];
        let mut count = 0;
        while i < n && nums[i] == value {
            count += 1;
            i += 1;
        }

        if top.len() < k {
            let pos = top.partition_point(|&(_, c)| c < count);
            top.insert(pos, (value, count));
        } else if count > top[0].1 {
            top.remove(0);
            let pos = top.partition_point(|&(_, c)| c < count);
            top.insert(pos, (value, count));
        }
    }

    top.reverse();
    top.into_iter().map(|(value, _)| value).collect()
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = frequent_elements(nums, k);
    println!("{:?}", result);
}
