use std::io::{
    self,
    // Write,
    BufRead,
    ErrorKind,
};
use std::fs::{
    // self,
    File
};

fn main() {
    let file_path = "to-do.txt";

    // Open and read file
    let list = match read_list(file_path) {
        Ok(l) => l,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("No list found. Starting from scratch...");
                // return Vec::<String>::new();
                return ();
            },
            other => {
                panic!("Problem opening list: {:?}", other);
            },
        },
    };

    print_list(&list);

    // Get user input
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        match input.trim() {
            "add" => {
                println!("Adding item");
            },
            "list" => {
                print_list(&list);
            },
            "exit" => {
                println!("Exiting");
                break
            },
            _ => println!("Invalid input!"),
        }
    }
}

fn read_list(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut ret: Vec<String> = Vec::new();

    for line in reader.lines() {
        ret.push(line?);
    }

    return Ok(ret);
}

fn print_list(l: &Vec<String>) {
    println!("\nTo do:");
    for item in l {
        println!("- {}", item);
    }
}