// Given an integer n, return a string array answer where:
// answer[i] = "Fizz" if divisible by 3, "Buzz" if divisible by 5, "FizzBuzz" if divisible by both, and i if by none.

// Test case

fn main() {
    println!("Example 1: {:?}", fizz_buzz(3));
    println!("Example 2: {:?}", fizz_buzz(5));
    println!("Example 3: {:?}", fizz_buzz(15));
}

// Submission (0ms beats 100%, 2.59 MB beats 98.39%)

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();
    for i in 1..(n + 1) {
        let mut text: String = String::new();
        if i % 3 == 0 {
            text.push_str("Fizz");
        }
        if i % 5 == 0 {
            text.push_str("Buzz");
        }
        if text == "" {
            text.push_str(i.to_string().as_str());
        }
        answer.push(text);
    }
    return answer;
}