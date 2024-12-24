use std::io::stdin;

fn main() {
    println!("Calculator");

    let input_one = convert_to_digit(get_input());
    let input_two = convert_to_digit(get_input());

    let operation = get_operation();

    match operation {
        Operation::Sum => println!("Sum ({input_one} + {input_two}): {}", input_one + input_two),
        Operation::Difference => println!("Difference ({input_one} - {input_two}): {}", input_one - input_two),
        Operation::Product => println!("Product ({input_one} * {input_two}): {}", input_one * input_two),
        Operation::Quotient => println!("Quotient ({input_one} / {input_two}): {}", input_one / input_two),
        Operation::Remainder => println!("Remainder ({input_one} % {input_two}): {}", input_one % input_two),
    }
}

enum Operation {
    Sum,
    Difference,
    Product,
    Quotient,
    Remainder,
}

fn get_operation() -> Operation {
    println!("Please input the operation (+ | - | * | / | %): ");
    let mut operation = String::new();

    stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    match operation.trim() {
        "+" => Operation::Sum,
        "-" => Operation::Difference,
        "*" => Operation::Product,
        "/" => Operation::Quotient,
        "%" => Operation::Remainder,
        _ => {
            println!("Invalid operation!");
            get_operation()
        }
    }
}

fn convert_to_digit(number: String) -> f64 {
    number.trim().parse().unwrap_or_else(|_| 0.0)
}

fn get_input() -> String {
    println!("Please input a number: ");
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}