// Given an integer array nums, return an array answer such that answer[i]
// is equal to the product of all the elements of nums except nums[i].

// Time complexity = O(n), space complexity = O(1) (but the output array doesn't count towards space).

// Test case

fn main() {
    println!("Example 1: {:?}", product_except_self(vec![1,2,3,4]));
    println!("Example 2: {:?}", product_except_self(vec![-1,1,0,-3,3]));
}

// Submission ()

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        let mut numbers: Vec<i32> = nums.clone();
        numbers.remove(i);
        answer.push(numbers.iter().product());
    }
    return answer;
}
