use dialoguer::{theme::ColorfulTheme, Select};
use std::io::stdin;

mod shapes;
use shapes::{Shape, SHAPES};

fn select_shape() -> Shape {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select shape:")
        .items(&SHAPES)
        .interact()
        .expect("Failed to select shape");

    let selected_shape = match selection {
        0 => Some(Shape::Circle),
        1 => Some(Shape::Ellipse),
        2 => Some(Shape::Parallelogram),
        3 => Some(Shape::Rectangle),
        4 => Some(Shape::Sector),
        5 => Some(Shape::Square),
        6 => Some(Shape::Trapezoid),
        7 => Some(Shape::Triangle),
        _ => None,
    };

    return selected_shape.unwrap();
}

fn main() {
    let shape = select_shape();

    if let Some(area) = shape.calculate() {
        println!("\nArea: {}", area);
    }

    println!("\n\n\n\nPress enter to exit...");
    stdin().read_line(&mut String::new()).unwrap();
}
