use std::io::{self, Write};

pub fn read_number(prompt: &str) -> f64 {
    let mut number = -1.0;

    while number < 0.0 {
        print!("\n{}: ", prompt);
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");

        match buffer.trim().parse() {
            Ok(num) => {
                if num < 0.0 {
                    println!("Must be bigger than 0");
                    continue;
                }

                number = num;
            }
            Err(_) => println!("Failed to parse input"),
        };
    }

    return number;
}
