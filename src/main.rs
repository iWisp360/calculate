mod functions;
use std::io;
use std::io::Write;
use std::process::exit;

enum Op {
    Sum,
    Sub,
    Mult,
    Div,
}

fn main() {
    println!("Welcome to my Calculator!");
    println!("What do you want to do?");
    print_help();
    // Start the CLI loop
    loop {
        print!("(calculator) ");
        io::stdout().flush().unwrap(); // Flush stdout to avoid late outputs

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        choice = choice.to_lowercase();
        // match user choice against available operation patterns
        match choice.trim() {
            "1" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty(Op::Sum, &num1, &num2));
            }
            "2" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty(Op::Sub, &num1, &num2));
            }
            "3" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty(Op::Mult, &num1, &num2));
            }
            "4" => {
                let (num1, num2) = input_constructor();
                println!("{}", operation_pretty(Op::Div, &num1, &num2));
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
    io::stdin().read_line(&mut val1).unwrap();

    print!("Type the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut val2).unwrap();

    (val1, val2)
}

fn operation(op: &Op, num1: &str, num2: &str) -> f64 {
    let num1: f64 = num1.trim().parse::<f64>().unwrap_or(0.0);
    let num2: f64 = match num2.trim().parse::<f64>() {
        Ok(num2) => num2,
        Err(_) => match op {
            Op::Div => 1.0,
            _ => 0.0,
        },
    };
    match op {
        Op::Sum => functions::sum(num1, num2),
        Op::Sub => functions::substraction(num1, num2),
        Op::Mult => functions::multiplication(num1, num2),
        Op::Div => functions::division(num1, num2),
    }
}

fn operation_pretty(op: Op, num1: &str, num2: &str) -> String {
    let result = operation(&op, num1, num2);
    let operator = match op {
        Op::Sum => "+",
        Op::Sub => "-",
        Op::Mult => "*",
        Op::Div => "/",
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
