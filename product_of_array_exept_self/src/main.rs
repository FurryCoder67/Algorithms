fn array_product(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut answer = vec![1; len];
    let mut prefix = 1;
    for i in 0..len {
        answer[i] = prefix;
        prefix *= nums[i];
    }
    let mut suffix = 1;
    for i in (0..len).rev() {
        answer[i] *= suffix;
        suffix *= nums[i];
    }
    answer
}

fn main() {
    let nums1 = vec![1, 2, 3, 4];
    let result1 = array_product(nums1);
    println!("Test 1: {:?}", result1);
    assert_eq!(result1, vec![24, 12, 8, 6]);
    let nums2 = vec![-1, 1, 0, -3, 3];
    let result2 = array_product(nums2);
    println!("Test 2: {:?}", result2);
    assert_eq!(result2, vec![0, 0, 9, 0, 0]);
    let nums3 = vec![5, 2];
    let result3 = array_product(nums3);
    println!("Test 3: {:?}", result3);
    assert_eq!(result3, vec![2, 5]);
    println!("All tests passed!");
}