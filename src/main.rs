use std::io;

fn main() {
    println!("Welcome to the calculator");

    loop {
        // Input number 1
        let mut num1 = String::new();

        println!("Please enter the first number");

        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");

        let num1: f32 = num1.trim()
            .parse()
            .expect("Please input a number!");

        // Input operation
        println!("Please select an operation (+,-,*,/):");

        let mut operation = String::new();

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        // Input number 2
        let mut num2 = String::new();

        println!("Please enter the second number");

        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");

        let num2: f32 = num2.trim()
            .parse()
            .expect("Please input a number!");

        // Perform the operation
        let result = match operation.trim() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                    continue;
                }
                num1 / num2
            },
            _ => {
                println!("Invalid operation selected.");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
