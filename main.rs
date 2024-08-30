use std::io;

fn main() {
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input);
}