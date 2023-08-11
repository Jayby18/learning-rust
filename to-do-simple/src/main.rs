use std::io::{
    self,
    Write,
    BufRead,
};
use std::fs::{
    self,
    File
};

fn main() -> io::Result<()> {
    let file_path = "to-do.txt";
    let _stdin = io::stdin;
    if !fs::metadata(file_path).is_ok() {
        File::create(file_path)?;
    }
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut list: Vec<String> = Vec::new();

    println!("\nTo do:");

    for line in reader.lines() {
        println!("- {}", line?);
        list.push(line);
    }

    println!("\n");

    while true {
        let mut user_input = String::new();

        _stdin.read_line(&mut user_input).expect("Failed to read line");

        if user_input == "add item" {
            println!("Added");
        }
        else if user_input == "exit" {
            break
        } else {
            println!("Not a valid response");
        }
    }

    Ok(())
}
