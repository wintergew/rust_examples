use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> Result<f64, &'static str> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b != 0.0 {
                Ok(a / b)
            } else {
                Err("Cannot divide by zero!")
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let operand1: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    input.clear();

    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let operation_str = input.trim().to_lowercase();

    let operation = match operation_str.as_str() {
        "add" => Operation::Add,
        "subtract" => Operation::Subtract,
        "multiply" => Operation::Multiply,
        "divide" => Operation::Divide,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    input.clear();

    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let operand2: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let result = calculate(operation(operand1, operand2));

    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("{}", error),
    }
}
