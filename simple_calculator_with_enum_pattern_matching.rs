enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator: Operator, operand1: i32, operand2: i32) -> Option<i32> {
    match operator {
        Operator::Add => Some(operand1 + operand2),
        Operator::Subtract => Some(operand1 - operand2),
        Operator::Multiply => Some(operand1 * operand2),
        Operator::Divide => {
            if operand2 != 0 {
                Some(operand1 / operand2)
            } else {
                None
            }
        }
    }
}

fn main() {
    let operand1 = 10;
    let operand2 = 5;

    let result_addition = calculate(Operator::Add, operand1, operand2);
    let result_subtraction = calculate(Operator::Subtract, operand1, operand2);
    let result_multiplication = calculate(Operator::Multiply, operand1, operand2);
    let result_division = calculate(Operator::Divide, operand1, operand2);

    match result_addition {
        Some(result) => println!("Addition: {}", result),
        None => println!("Cannot divide by zero!"),
    }

    match result_subtraction {
        Some(result) => println!("Subtraction: {}", result),
        None => println!("Cannot divide by zero!"),
    }

    match result_multiplication {
        Some(result) => println!("Multiplication: {}", result),
        None => println!("Cannot divide by zero!"),
    }

    match result_division {
        Some(result) => println!("Division: {}", result),
        None => println!("Cannot divide by zero!"),
    }
}
