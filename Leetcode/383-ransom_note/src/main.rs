// Given two string ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
// Each leter in magazine can only be used once in ransomNote

// Test case
fn main() {
    println!("1: {:?}", can_construct(String::from("a"), String::from("b")));
    // println!("2: {:?}", can_construct("aa", "ab"));
    // println!("3: {:?}", can_construct("aa", "aab"));
}

// Submission
fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut answer: bool = true;
    for letter in ransom_note.chars() {
        if magazine.contains(letter) {
            println!("Letter present");
            
        } else {
            println!("Letter not present");
            answer = false;
            return answer;
        }
    }
    return answer;
}