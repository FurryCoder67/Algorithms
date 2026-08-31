// given an array of integers and a target value, return the indices of the two numbers that add up to the target
fn two_sum(nums: Vec<i32>, target: i32) -> (usize, usize) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return (i, j);
            }
        }
    }
    panic!("No two sum solution");
}

// testing
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let (i, j) = two_sum(nums, target);
    println!("Indices: ({}, {})", i, j);
}