fn main() {
    assert_eq!(two_sum([2,7,11,15].to_vec(), 9), [0, 1]);
    assert_eq!(two_sum([3,2,4].to_vec(), 6), [1,2]);
    assert_eq!(two_sum([3,3].to_vec(), 6), [0,1]);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        for n in 0..nums.len() {
            if nums[i] + nums[n] == target && i != n {
                ret.push(i as i32);
                ret.push(n as i32);
                return ret;
            }
        }
    }
    return ret;
}

