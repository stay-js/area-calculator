mod shapes;

use dialoguer::{theme::ColorfulTheme, Select};
use std::io::stdin;

fn main() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select shape:")
        .items(&shapes::SHAPES)
        .interact()
        .expect("Failed to select shape");

    if let Some(area) = match selection {
        0 => Some(shapes::circle::calculate()),
        1 => Some(shapes::ellipse::calculate()),
        2 => Some(shapes::parallelogram::calculate()),
        3 => Some(shapes::rectangle::calculate()),
        4 => Some(shapes::regular_polygon::calculate()),
        5 => Some(shapes::sector::calculate()),
        6 => Some(shapes::square::calculate()),
        7 => Some(shapes::trapezoid::calculate()),
        8 => shapes::triangle::calculate(),
        _ => None,
    } {
        println!("\nArea: {}", area);
    }

    println!("\n\n\n\nPress enter to exit...");
    stdin().read_line(&mut String::new()).unwrap();
}
