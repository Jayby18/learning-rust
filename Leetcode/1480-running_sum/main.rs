// Test case

fn main() {
    let nums1: Vec<i32> = vec![1, 2, 3, 4];
    let nums2: Vec<i32> = vec![1, 1, 1, 1, 1];
    let nums3: Vec<i32> = vec![3, 1, 2, 10, 1];
    println!("Example 1: {:?}", running_sum(&nums1));
    println!("Example 2: {:?}", running_sum(&nums2));
    println!("Example 3: {:?}", running_sum(&nums3));
}

// Submission 1 (0 ms beats 100%, 2.3 MB beats 13.2%)

fn running_sum(nums: &Vec<i32>) -> Vec<i32> {
    let mut sum: Vec<i32> = Vec::new();
    sum.push(nums[0]);
    for i in 1..(nums.len()) {
        sum.push(sum[i - 1] + nums[i]);
    }
    return sum;
}
