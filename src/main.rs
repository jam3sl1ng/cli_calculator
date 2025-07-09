use std::io;

fn main() {
    println!("Welcome to the calculator");

    println!("Please enter the first number");

    let mut num1 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    let num1: f32 = num1.trim()
        .parse()
        .expect("Please input a number!");

    println!("You entered: {}", num1);
}
