// given an array of integers and a target value, return the indices of the two numbers that add up to the target
fn two_sum(nums: Vec<i32>, target: i32) -> (usize, usize) {
    let mut num_indices = std::collections::HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = num_indices.get(&complement) {
            return (j, i);
        }
        num_indices.insert(num, i);
    }
    panic!("No two sum solution found");
}

// testing
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let (i, j) = two_sum(nums, target);
    println!("Indices: ({}, {})", i, j);
}