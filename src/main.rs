use std::io::{self, Write};

mod shapes;
use shapes::Shape;

fn select_shape() -> Shape {
    println!(
        "
Shapes:
\t1 - Circle
\t2 - Ellipse
\t3 - Parallelogram
\t4 - Rectangle
\t5 - Sector
\t6 - Square
\t7 - Trapezoid
\t8 - Triangle"
    );

    let mut selected_shape: Option<Shape> = None;

    while selected_shape.is_none() {
        print!("\nSelect shape: ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");

        selected_shape = match buffer.trim() {
            "1" => Some(Shape::Circle),
            "2" => Some(Shape::Ellipse),
            "3" => Some(Shape::Parallelogram),
            "4" => Some(Shape::Rectangle),
            "5" => Some(Shape::Sector),
            "6" => Some(Shape::Square),
            "7" => Some(Shape::Trapezoid),
            "8" => Some(Shape::Triangle),
            _ => None,
        };

        if selected_shape.is_none() {
            println!("Please input a number between 1 and 8")
        }
    }

    return selected_shape.unwrap();
}

fn main() {
    let shape = select_shape();
    println!();

    if let Some(area) = shape.calculate() {
        println!("\nArea: {}", area);
    }
}
