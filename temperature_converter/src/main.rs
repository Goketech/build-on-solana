// Import the standard input/output library
use std::io;

// Main function - entry point of the program
fn main() {
    // Print a welcome message to the console
    println!("Temperature Converter");
    println!("====================\n");

    // Loop indefinitely until user chooses to exit
    loop {
        // Display menu options
        println!("Select conversion type:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");
        print!("Enter your choice (1-3): ");

        // Create a new mutable String to store user input
        // 'mut' keyword makes the variable mutable (can be changed)
        let mut choice = String::new();

        // Read user input from stdin
        // read_line() returns a Result type which can be Ok or Err
        // expect() handles potential errors and panics with the message if it fails
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // trim() removes whitespace and newline characters
        // parse() converts the string to a number (u32 = unsigned 32-bit integer)
        // The underscore in Ok(_) ignores the success value
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,      // If parsing succeeds, use the number
            Err(_) => {          // If parsing fails, continue to next iteration
                println!("Invalid input! Please enter a number.\n");
                continue;
            }
        };

        // Match expression - similar to switch/case in other languages
        match choice {
            1 => {
                // Call function to convert Fahrenheit to Celsius
                fahrenheit_to_celsius();
            }
            2 => {
                // Call function to convert Celsius to Fahrenheit
                celsius_to_fahrenheit();
            }
            3 => {
                // Exit the program
                println!("Goodbye!");
                break;  // Break out of the loop
            }
            _ => {
                // Default case - handles any other number
                println!("Invalid choice! Please select 1, 2, or 3.\n");
            }
        }
    }
}

// Function to convert Fahrenheit to Celsius
// No parameters or return value - handles I/O internally
fn fahrenheit_to_celsius() {
    println!("\n--- Fahrenheit to Celsius ---");
    
    // Get temperature input from user
    let fahrenheit = get_temperature_input("Enter temperature in Fahrenheit: ");
    
    // Formula: C = (F - 32) * 5/9
    // Using 5.0/9.0 for floating-point division
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    
    // Print result with 2 decimal places using formatting
    // {:.2} means format as floating-point with 2 decimal places
    println!("{:.2}°F = {:.2}°C\n", fahrenheit, celsius);
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit() {
    println!("\n--- Celsius to Fahrenheit ---");
    
    // Get temperature input from user
    let celsius = get_temperature_input("Enter temperature in Celsius: ");
    
    // Formula: F = C * 9/5 + 32
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    
    // Print result with 2 decimal places
    println!("{:.2}°C = {:.2}°F\n", celsius, fahrenheit);
}

// Helper function to get temperature input from user
// Takes a prompt message as parameter
// Returns f64 (64-bit floating-point number)
// The -> f64 syntax indicates the return type
fn get_temperature_input(prompt: &str) -> f64 {
    // Loop until valid input is received
    loop {
        // Print the prompt without a newline (using print! instead of println!)
        print!("{}", prompt);
        
        // Flush stdout to ensure the prompt is displayed immediately
        // Without this, the prompt might not appear before input
        use std::io::Write;
        io::stdout().flush().unwrap();
        
        // Create mutable String for input
        let mut input = String::new();
        
        // Read the line from user
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Try to parse the input as f64
        // trim() removes whitespace/newlines first
        match input.trim().parse::<f64>() {
            Ok(temp) => return temp,  // If successful, return the temperature
            Err(_) => {                // If it fails, show error and loop again
                println!("Invalid input! Please enter a valid number.");
            }
        }
    }
}
