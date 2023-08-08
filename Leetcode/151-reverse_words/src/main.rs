// Given an input string s, reverse the order of words. Exclude extra whitespaces.

// Test case

fn main() {
    println!("Example 1: '{}'", reverse_words("the sky is blue".to_string()));
    println!("Example 2: '{}'", reverse_words("  hello world  ".to_string()));
    println!("Example 3: '{}'", reverse_words("a good   example".to_string()));
}

// Submission (1 ms beats 84.56%, 2.14 MB beats 55.03%)

fn reverse_words(s: String) -> String {
    let mut new_string: String = String::new();
    let words: Vec<&str> = s.split_whitespace().collect();
    for word in words.iter().rev() {
        new_string.push_str(word);
        new_string.push(' ');
    }
    new_string.pop();
    return new_string;
}