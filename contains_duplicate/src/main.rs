// if the array contains any value that appears twice, return true, else return false
fn contains_duplicate(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }
    return false;
}

//tests
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("{}", contains_duplicate(nums));
}