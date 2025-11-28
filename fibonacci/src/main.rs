// Import the standard input/output library
use std::io;

// Main function - entry point of the program
fn main() {
    // Print a welcome message
    println!("Fibonacci Number Generator");
    println!("==========================\n");

    // Infinite loop to allow multiple calculations
    loop {
        println!("Choose a method to generate Fibonacci number:");
        println!("1. Iterative (Fast, recommended)");
        println!("2. Recursive (Educational, slow for large numbers)");
        println!("3. Exit");
        print!("Enter your choice (1-3): ");

        // Create a mutable String to store the choice
        let mut choice = String::new();
        
        // Flush stdout to ensure prompt appears before input
        use std::io::Write;
        io::stdout().flush().unwrap();

        // Read user input
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Parse the choice as an unsigned 32-bit integer
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.\n");
                continue;  // Skip to next loop iteration
            }
        };

        // Handle the user's choice
        match choice {
            1 => {
                // Get the nth position from user
                let n = get_position_input();
                // Calculate using iterative method
                let result = fibonacci_iterative(n);
                println!("The {}th Fibonacci number is: {}\n", n, result);
            }
            2 => {
                let n = get_position_input();
                // Warn user about performance for large numbers
                if n > 40 {
                    println!("Warning: Recursive method is slow for n > 40. This may take a while...");
                }
                // Calculate using recursive method
                let result = fibonacci_recursive(n);
                println!("The {}th Fibonacci number is: {}\n", n, result);
            }
            3 => {
                println!("Goodbye!");
                break;  // Exit the loop and end program
            }
            _ => {
                println!("Invalid choice! Please select 1, 2, or 3.\n");
            }
        }
    }
}

// Helper function to get the position (n) from the user
// Returns a u64 (unsigned 64-bit integer) to handle large numbers
fn get_position_input() -> u64 {
    loop {
        print!("Enter which Fibonacci number you want (n >= 0): ");
        
        // Flush stdout so the prompt appears immediately
        use std::io::Write;
        io::stdout().flush().unwrap();
        
        // Create a mutable String to store input
        let mut input = String::new();
        
        // Read the line from stdin
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Try to parse the input as u64
        // The turbofish syntax ::<u64> explicitly specifies the type to parse to
        match input.trim().parse::<u64>() {
            Ok(n) => return n,  // Return the number if parsing succeeds
            Err(_) => {
                println!("Invalid input! Please enter a valid non-negative number.");
            }
        }
    }
}

// Iterative approach to calculate Fibonacci numbers
// This is MUCH faster than recursion, especially for large numbers
// Time Complexity: O(n) - linear time
// Space Complexity: O(1) - constant space
fn fibonacci_iterative(n: u64) -> u64 {
    // Handle base cases
    // The Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...
    // F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2)
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // Initialize the first two Fibonacci numbers
    // 'mut' makes these variables mutable so we can update them in the loop
    let mut prev = 0;      // F(0)
    let mut current = 1;   // F(1)
    
    // Iterate from 2 to n
    // 2..=n creates an inclusive range (includes n)
    // 2..n would exclude n (exclusive range)
    for _ in 2..=n {
        // Calculate the next Fibonacci number
        let next = prev + current;
        
        // Shift the values: current becomes prev, next becomes current
        prev = current;
        current = next;
        
        // This loop continues building up the sequence:
        // prev=0, current=1 -> next=1
        // prev=1, current=1 -> next=2
        // prev=1, current=2 -> next=3
        // prev=2, current=3 -> next=5
        // and so on...
    }
    
    // Return the final current value, which is F(n)
    current
}

// Recursive approach to calculate Fibonacci numbers
// This is educational but VERY slow for large numbers
// Time Complexity: O(2^n) - exponential time (very slow!)
// Space Complexity: O(n) - due to call stack depth
// 
// Why is it slow? Because it recalculates the same values many times
// Example: fib(5) calls fib(4) and fib(3)
//          fib(4) calls fib(3) and fib(2)
//          So fib(3) is calculated twice, fib(2) is calculated 3 times, etc.
fn fibonacci_recursive(n: u64) -> u64 {
    // Base cases - these stop the recursion
    // Without base cases, recursion would continue forever (stack overflow!)
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // Recursive case: F(n) = F(n-1) + F(n-2)
    // This function calls itself twice with smaller values
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

// Note for learning:
// There's also a third approach called "memoization" or "dynamic programming"
// which stores previously calculated values in a HashMap or Vec to avoid
// recalculating them. This combines the elegance of recursion with the
// speed of iteration. You can explore this as an advanced exercise!
