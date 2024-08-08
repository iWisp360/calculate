mod functions;
use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    println!("Welcome to my Calculator!");
    println!("What do you want to do?");
    print_help();
    // Start the CLI loop
    loop {
        print!("(calculator) ");
        io::stdout().flush().unwrap(); // Flush stdout to avoid late outputs

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Whoops!");
        choice = choice.to_lowercase();
        // match user choice against available operation patterns
        match choice.trim() {
            "1" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty("sum", &num1, &num2));
            }
            "2" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty("sub", &num1, &num2));
            }
            "3" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty("mult", &num1, &num2));
            }
            "4" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty("div", &num1, &num2));
            }
            "h" => print_help(),
            "" => (),
            "e" => exit(0),
            _ => println!("Please enter a valid input!"),
        }
    }
}

fn print_help() {
    println!(
        "(1) Sum
(2) Substraction
(3) Multiplication
(4) Division
(h) Help
(e) Exit"
    );
}

fn input_constructor() -> (String, String) {
    let mut val1 = String::new();
    let mut val2 = String::new();

    print!("Type the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut val1).expect("Whoops!");

    print!("Type the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut val2).expect("Whoops!");

    (val1, val2)
}

fn operation(op: &str, num1: &str, num2: &str) -> isize {
    let num1: isize = num1.trim().parse().expect("Not a number!");
    let num2: isize = num2.trim().parse().expect("Not a number!");
    if op == "sum" {
        functions::sum(num1, num2)
    } else if op == "sub" {
        functions::substraction(num1, num2)
    } else if op == "mult" {
        functions::multiplication(num1, num2)
    } else if op == "div" {
        functions::division(num1, num2)
    } else {
        panic!("No valid operation received!");
    }
}

fn operation_pretty(op: &str, num1: &str, num2: &str) -> String {
    let result = operation(op, num1, num2);
    let operator = match op {
        "sum" => "+",
        "sub" => "-",
        "mult" => "*",
        "div" => "/",
        _ => "",
    };
    // Return formatted string
    format!(
        "The Result for {} {} {} is {}",
        num1.trim(),
        operator,
        num2.trim(),
        result
    )
}
