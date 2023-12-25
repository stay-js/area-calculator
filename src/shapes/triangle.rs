use dialoguer::{theme::ColorfulTheme, Select};
use lib::read_number::read_number;

fn calculate_from_base_and_height() -> f64 {
    let base = read_number("Base:");
    let height = read_number("Height:");

    return base * height / 2.0;
}

fn calculate_from_sides() -> Option<f64> {
    let a = read_number("\"a\" side:");
    let b = read_number("\"b\" side:");
    let c = read_number("\"c\" side:");

    if a + b <= c || a + c <= b || b + c <= a {
        println!("\nInvalid triangle");
        return None;
    }

    let s = (a + b + c) / 2.0;

    return Some((s * (s - a) * (s - b) * (s - c)).sqrt());
}

const METHODS: [&str; 2] = ["Base and height", "Sides"];

pub fn calculate() -> Option<f64> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Calculate area from:")
        .items(&METHODS)
        .interact()
        .expect("Failed to select method");

    return match selection {
        0 => Some(calculate_from_base_and_height()),
        1 => calculate_from_sides(),
        _ => None,
    };
}
