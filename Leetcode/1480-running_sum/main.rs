impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum: [i32, nums.len()];
        sum[0] = nums[0];
        for i in 1..(nums.len()) {
            sum[i] = sum[i - 1] + nums[i];
        }
        return sum;
    }
}

fn main() {
    let nums = [1,2,3,4];
    let sum = running_sum(nums);
    print(sum);
}
