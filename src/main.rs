mod shapes;

use dialoguer::{theme::ColorfulTheme, Select};
use std::io::stdin;

fn main() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select shape:")
        .items(&shapes::SHAPES)
        .interact()
        .expect("Failed to select shape");

    let (area, perimeter) = match selection {
        0 => shapes::circle::calculate(),
        1 => shapes::ellipse::calculate(),
        2 => shapes::parallelogram::calculate(),
        3 => shapes::rectangle::calculate(),
        4 => shapes::regular_polygon::calculate(),
        5 => shapes::sector::calculate(),
        6 => shapes::square::calculate(),
        7 => shapes::trapezoid::calculate(),
        8 => shapes::triangle::calculate(),
        _ => (None, None),
    };

    println!();

    if let Some(area) = area {
        println!("Area: {}", area);
    }

    if let Some(perimeter) = perimeter {
        println!("Perimeter: {}", perimeter);
    }

    println!("\n\n\n\nPress enter to exit...");
    stdin().read_line(&mut String::new()).unwrap();
}
