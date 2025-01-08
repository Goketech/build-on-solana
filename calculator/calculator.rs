use std::io;


fn main () {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num1: f64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number input. Please restart the program.");
            return;
        }
    };

    println!("Enter operation (+, -, *, /):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let op = input.trim().to_string();

    println!("Enter the second number:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num2: f64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number input. Please restart the program.");
            return;
        }
    };

    let operation_instance = match op.as_str() {
        "+" => Operation::Add (num1, num2),
        "-" => Operation::Subtract (num1, num2),
        "*" => Operation::Multiply (num1, num2),
        "/" => Operation::Divide (num1, num2),
        _ => {
            eprintln!("Invalid operation. Please restart the program.");
            return;
        }
    };

    let result = operation_instance.calculate();

    println!("Result: {}", result);
}

impl Operation {
    fn calculate(&self) -> f64 {
        match self {
            Operation::Add (a, b) => a + b,
            Operation::Subtract (a, b) => a - b,
            Operation::Multiply (a, b) => a * b,
            Operation::Divide (a, b) => {
                if *b == 0.0 {
                    eprintln!("Cannot divide by zero. Please restart the program.");
                    return 0.0;
                }
                a / b
            },
        }
    }
}

enum Operation {
    Add (f64, f64),
    Subtract (f64, f64),
    Multiply (f64, f64),
    Divide (f64, f64),
}