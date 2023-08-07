// You are given an m x n integer grid accounts where accounts[i][j] is the amount of money the i​​​​​​​​​​​th​​​​ customer has in the j​​​​​​​​​​​th​​​​ bank.
// Return the wealth that the richest customer has.
// 
// A customer's wealth is the amount of money they have in all their bank accounts.
// The richest customer is the customer that has the maximum wealth.

// Test case

fn main() {
    let accounts1 = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let accounts2 = vec![vec![1,5],vec![7,3],vec![3,5]];
    let accounts3 = vec![vec![2,8,7],vec![7,1,3],vec![1,9,5]];
    println!("Maximum wealth 1: {}", maximum_wealth(accounts1));
    println!("Maximum wealth 2: {}", maximum_wealth(accounts2));
    println!("Maximum wealth 3: {}", maximum_wealth(accounts3));
}

// Submission (1 ms beats 84.47%, 2.10 mb beats 86.36%)

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max_wealth = 0;
    for customer in accounts {
        let mut wealth = 0;
        for bank in customer {
            wealth += bank;
        }
        if wealth > max_wealth {
            max_wealth = wealth;
        }
    }
    return max_wealth;
}
