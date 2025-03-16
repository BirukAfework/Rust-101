use std::io; // Import the standard input/output library

fn main() {
    // Print a welcome message
    println!("Simple 4-Ops Calculator");
    println!("Enter the first number:");

    // Create a mutable string to store the first number input
    let mut num1 = String::new();

    // Read input from the user and store it in `num1`
    io::stdin()
        .read_line(&mut num1) // Read the input line
        .expect("Failed to read input"); // Handle potential errors

    // Convert the input string to a floating-point number (`f64`)
    let num1: f64 = num1
        .trim() // Remove leading/trailing whitespace
        .parse() // Parse the string into a number
        .expect("Please enter a valid number"); // Handle parsing errors

    // Prompt the user to enter the operation
    println!("Enter the operation (+, -, *, /):");

    // Create a mutable string to store the operation input
    let mut op = String::new();

    // Read input from the user and store it in `op`
    io::stdin()
        .read_line(&mut op) // Read the input line
        .expect("Failed to read input"); // Handle potential errors

    // Remove whitespace from the operation input
    let op = op.trim();

    // Prompt the user to enter the second number
    println!("Enter the second number:");

    // Create a mutable string to store the second number input
    let mut num2 = String::new();

    // Read input from the user and store it in `num2`
    io::stdin()
        .read_line(&mut num2) // Read the input line
        .expect("Failed to read input"); // Handle potential errors

    // Convert the input string to a floating-point number (`f64`)
    let num2: f64 = num2
        .trim() // Remove leading/trailing whitespace
        .parse() // Parse the string into a number
        .expect("Please enter a valid number"); // Handle parsing errors

    // Perform the calculation based on the operation
    let result = match op {
        "+" => num1 + num2, // Addition
        "-" => num1 - num2, // Subtraction
        "*" => num1 * num2, // Multiplication
        "/" => {
            // Check for division by zero
            if num2 == 0.0 {
                println!("Error: Division by zero!"); // Print an error message
                return; // Exit the program early
            } else {
                num1 / num2 // Perform division
            }
        }
        _ => {
            // Handle invalid operations
            println!("Invalid operation!"); // Print an error message
            return; // Exit the program early
        }
    };

    // Print the result of the calculation
    println!("Result: {}", result);
}
