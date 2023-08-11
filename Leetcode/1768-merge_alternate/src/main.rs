// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1.
// If a string is longer than the other, append the additional letters onto the end of the merged string.
// Return the merged string.

// Test case

fn main() {
    println!("1: {:?}", merge_alternately("abc".to_string(), "pqr".to_string()));
    println!("2: {:?}", merge_alternately("ab".to_string(), "pqrs".to_string()));
    println!("3: {:?}", merge_alternately("abcd".to_string(), "pq".to_string()));
}

// Submission (1 ms beats 78.36%, 2.08 MB beats 64.33%)

fn merge_alternately(word1: String, word2: String) -> String {
    let mut result: String = String::new();
    let mut string1: String = word1;
    let mut string2: String = word2;
    if string1.len() > string2.len() {
        // string1 is longer than word2
        for i in 0..(string1.len() - string2.len()) {
            string2.push_str(" ");
        }
    } else if string1.len() < string2.len() {
        // string2 is longer than word1
        for i in 0..(string2.len() - string1.len()) {
            string1.push_str(" ");
        }
    }

    for i in 0..(string1.len()) {
        result.push(string1.chars().nth(i).unwrap_or('?'));
        result.push(string2.chars().nth(i).unwrap_or('?'));
    }

    result.replace(" ", "")
}