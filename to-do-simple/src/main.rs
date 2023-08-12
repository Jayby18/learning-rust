use std::io::{
    self,
    Write,
    BufRead,
    ErrorKind,
};
use std::fs::{
    self,
    File
};

fn main() {
    let file_path = "to-do.txt";
    let _stdin = io::stdin;

    let list = match load_list(file_path) {
        Ok(l) => {
            println!("\nTo do:");
            for item in l {
                println!("- {}", item);
            }
            return l;
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("No to-do's found. Starting from empty list...");
                return Vec::new();
            },
            other => panic!("Problem opening file: {:?}", other),
        }
    };

    // loop {
    //     let mut user_input = String::new();

    //     _stdin().read_line(&mut user_input).expect("Failed to read line");
    //     println!("Input: {}", user_input);

    //     if user_input == "add item" {
    //         println!("Added");
    //     } else if user_input == "exit" {
    //         println!("Goodbye!");
    //         break
    //     } else {
    //         println!("Not a valid response");
    //         break
    //     }
    // }
}

fn load_list(path: &str) -> Result<Vec<String>, io::Error> {
    // Open and read file
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Create empty list
    let mut list: Vec<String> = Vec::new();

    // Add each line to list
    for line in reader.lines() {
        let new_line: String = line;
        list.push(new_line);
    }
    list
}

// fn save_list(path: &str) -> Result<> {

// }
