use std::io;

fn main() {
    println!("Enter the first string: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    
    println!("Enter the second string: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    println!("You entered: {} and {}", input1.trim(), input2.trim());
}