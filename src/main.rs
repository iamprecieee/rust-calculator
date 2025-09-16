use std::io::{self, Write};

fn main() {
    println!("Welcome to multi-function calculator🤖!\n");
    
    const VALID_OPERATIONS: [&str; 4] = [
        "add",
        "subtract",
        "multiply",
        "divide",
    ];

    let mut choices = Vec::new();
    for (i, operation) in VALID_OPERATIONS.iter().enumerate() {
        choices.push((i, operation));
    }

    'main: loop {
        println!("Select an operation from the options below.");
        for (i, operation) in &choices {
            println!("{}. {}", i + 1, operation);
        }

        print!("Reply with a numeric value (e.g. 1, 2, 3): ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let operation_index = match input.trim().parse::<usize>() {
            Ok(num) => {
                if num > choices.len()  {
                    println!("Please enter a number between 1 and {}.", choices.len());
                    continue;
                } else{
                    num
                }
            },
            Err(..) => {
                print!("Please enter a valid number: ");
                io::stdout().flush().unwrap();
                continue;
            }
        };

        let mut vec_values = get_math_values();
        let vec_values_str = vec_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        match *choices[(operation_index) - 1].1 {
            "add" => {
                let sum_value = add(&vec_values);
                println!("The sum of [{}] is {}.", vec_values_str, sum_value);
            },
            "subtract" => {
                let difference_value = subtract(&mut vec_values);
                println!("The difference of [{}] is {}.", vec_values_str, difference_value);
            },
            "multiply" => {
                let product_value = multiply(&mut vec_values);
                println!("The product of [{}] is {}.", vec_values_str, product_value);
            },
            "divide" => {
                let quotient_value = divide(&mut vec_values);
                println!("The quotient of [{}] in order is {}.", vec_values_str, quotient_value);
            },
            _ => {
                println!("Invalid operation detected.");
                continue;
            }
        };

        let mut proceed = String::new();

        print!("Do you want to perform another operation? (yes/no) ");
        io::stdout().flush().unwrap();

        loop {
            io::stdin().read_line(&mut proceed).expect("Failed to read line.");
            
            match proceed.trim().to_lowercase().as_str() {
                "no" => break 'main,
                "yes" => break,
                _ => {
                    print!("Enter a value 'yes' or 'no': ");
                    io::stdout().flush().unwrap();
                    proceed.clear();
                }
            }
        }
    }
}

fn get_math_values() -> Vec<f32> {
    let mut num_values = Vec::new();

    print!("Enter the first value: ");
    io::stdout().flush().unwrap();
    'outer: loop {
        let mut num_value: String = String::new();
        io::stdin().read_line(&mut num_value).expect("Failed to read line.");
        let num_value = match num_value.trim().parse::<f32>() {
            Ok(num) => num,
            Err(..) => {
                print!("Please enter a valid number: ");
                io::stdout().flush().unwrap();
                continue;
            }
        };

        num_values.push(num_value);

        print!("Is there another value? (yes/no) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        loop {
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            
            match input.trim().to_lowercase().as_str() {
                "no" => break 'outer,
                "yes" => break,
                _ => {
                    print!("Enter a value 'yes' or 'no': ");
                    io::stdout().flush().unwrap();
                    input.clear();
                }
            }
        }

        print!("Enter the next value: ");
        io::stdout().flush().unwrap();

    }

    return num_values;
}

fn add(num_value_list: &Vec<f32>) -> f32 {
    return num_value_list.iter().sum();
}

fn subtract(num_value_list: &mut Vec<f32>) -> f32 {
    let mut first = num_value_list[0];
    let num_list_length = num_value_list.len();

    if num_list_length > 0 {
        for x in 1..num_list_length {
            first -= num_value_list[x];
        }
    }

    return first;
}

fn multiply(num_value_list: &mut Vec<f32>) -> f32 {
    return num_value_list.iter().product();
}

fn divide(num_value_list: &mut Vec<f32>) -> f32 {
    let mut first = num_value_list[0];
    let num_list_length = num_value_list.len();

    if num_list_length > 0 {
        for x in 1..num_list_length {
            first /= num_value_list[x];
        }
    }

    return first;
}