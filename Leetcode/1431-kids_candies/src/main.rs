// There are n kids with candies. You are given an integer array candies, where each candies[i] represents the number of candies the ith kid has,
// and an integer extraCandies, denoting the number of extra candies that you have.

// Return a boolean array result of length n, where result[i] is true if, after giving the ith kid all the extraCandies,
// they will have the greatest number of candies among all the kids, or false otherwise.

// Note that multiple kids can have the greatest number of candies.

// Test case

fn main() {
    println!("Output:\n");
    println!("{:?}", kids_with_candies(vec![2,3,5,1,3], 3));
    println!("{:?}", kids_with_candies(vec![4,2,1,1,2], 1));
    println!("{:?}", kids_with_candies(vec![12,1,12], 10));
}

// Submission (1 ms, 2.13 MB)

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    for kid in &candies {
        let mut most = true;
        let current = kid + extra_candies;
        for other in &candies {
            if current < *other {
                most = false;
            }
        }
        result.push(most);
    }
    result
}
