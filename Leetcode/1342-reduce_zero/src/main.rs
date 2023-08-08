// Given an integer num, return the number of steps to reduce it to zero.

// In one step, if the current number is even, you have to divide it by 2, otherwise you have to subtract 1 from it.

// Test case

fn main() {
    println!("1: {:?}", number_of_steps(14));
    println!("2: {:?}", number_of_steps(8));
    println!("3: {:?}", number_of_steps(123));
}

// Submission (0 ms beats 100%, 2.06 MB beats 47.85%)

fn number_of_steps(num: i32) -> i32 {
    let mut current: i32 = num;
    let mut count: i32 = 0;
    while current != 0 {
        if current % 2 == 0 {
            current = current / 2;
        } else {
            current -= 1;
        }
        count += 1;
    }
    return count;
}
