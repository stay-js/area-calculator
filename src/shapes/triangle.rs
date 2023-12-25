use lib::read_number::read_number;
use std::io::{self, Write};

enum Method {
    FromBaseAndHeight,
    FromSides,
}

fn select_method() -> Method {
    println!(
        "
Methods:
\t1 - Calculate Area from base and height
\t2 - Calculate Area from sides"
    );

    let mut selected_method: Option<Method> = None;

    while selected_method.is_none() {
        print!("\nSelect method: ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");

        selected_method = match buffer.trim() {
            "1" => Some(Method::FromBaseAndHeight),
            "2" => Some(Method::FromSides),
            _ => None,
        };

        if selected_method.is_none() {
            println!("Please input a number between 1 and 2")
        }
    }

    return selected_method.unwrap();
}

fn calculate_from_base_and_height() -> f64 {
    let base = read_number("Please provide the base");
    let height = read_number("Please provide the height");

    return base * height / 2.0;
}
fn calculate_from_sides() -> f64 {
    let a = read_number("Please provide the \"a\" side");
    let b = read_number("Please provide the \"b\" side");
    let c = read_number("Please provide the \"c\" side");

    let s = (a + b + c) / 2.0;

    return (s * (s - a) * (s - b) * (s - c)).sqrt();
}

pub fn calculate() -> f64 {
    return match select_method() {
        Method::FromBaseAndHeight => calculate_from_base_and_height(),
        Method::FromSides => calculate_from_sides(),
    };
}
