use dialoguer::{theme::ColorfulTheme, Select};
use lib::read_number::read_number;

fn calculate_from_base_and_height() -> (Option<f64>, Option<f64>) {
    let base = read_number("Base:");
    let height = read_number("Height:");

    let area = base * height / 2.0;

    return (Some(area), None);
}

fn calculate_from_sides() -> (Option<f64>, Option<f64>) {
    let a = read_number("\"a\" side:");
    let b = read_number("\"b\" side:");
    let c = read_number("\"c\" side:");

    if a + b <= c || a + c <= b || b + c <= a {
        println!("\nInvalid triangle");
        return (None, None);
    }

    let s = (a + b + c) / 2.0;

    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
    let perimeter = a + b + c;

    return (Some(area), Some(perimeter));
}

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("To calculate:")
        .items(&[
            "Area (from base and height)",
            "Perimeter and Area (from sides)",
        ])
        .interact()
        .expect("Failed to select to calculate");

    return match selection {
        0 => calculate_from_base_and_height(),
        1 => calculate_from_sides(),
        _ => (None, None),
    };
}
