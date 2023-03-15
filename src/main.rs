use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // println!("{:?}", args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);


    let contents = fs::read_to_string(file_path).expect("Shouold read file");

    println!("With text:\n{contents}");
}