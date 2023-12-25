use std::io::{self, Write};

mod shapes;
use shapes::Shape;

// fn select_shape() -> Shape {
//     println!(
//         "
// Shapes:
// \t1 - Rectangle
// \t2 - Square
// \t3 - Triangle
// \t4 - Trapezoid
// \t5 - Circle
// \t6 - Sector
// \t7 - Ellipse
// \t8 - Parallelogram"
//     );

//     let mut selected_shape: Option<Shape> = None;

//     while selected_shape.is_none() {
//         print!("\nSelect shape: ");
//         io::stdout().flush().unwrap();

//         let mut buffer = String::new();
//         io::stdin()
//             .read_line(&mut buffer)
//             .expect("Failed to read input");

//         selected_shape = match buffer.trim() {
//             "1" => Some(Shape::Rectangle),
//             "2" => Some(Shape::Square),
//             "3" => Some(Shape::Triangle),
//             "4" => Some(Shape::Trapezoid),
//             "5" => Some(Shape::Circle),
//             "6" => Some(Shape::Sector),
//             "7" => Some(Shape::Ellipse),
//             "8" => Some(Shape::Parallelogram),
//             _ => None,
//         };

//         if selected_shape.is_none() {
//             println!("Please input a number between 1 and 8")
//         }
//     }

//     return selected_shape.unwrap();
// }

fn main() {
    println!(
        "
Shapes:
\t1 - Rectangle
\t2 - Square
\t3 - Triangle
\t4 - Trapezoid
\t5 - Circle
\t6 - Sector
\t7 - Ellipse
\t8 - Parallelogram"
    );

    let mut selected_shape: Option<Shape> = None;

    while selected_shape.is_none() {
        print!("Select shape: ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");

        selected_shape = match buffer.trim() {
            "1" => Some(Shape::Rectangle),
            "2" => Some(Shape::Square),
            "3" => Some(Shape::Triangle),
            "4" => Some(Shape::Trapezoid),
            "5" => Some(Shape::Circle),
            "6" => Some(Shape::Sector),
            "7" => Some(Shape::Ellipse),
            "8" => Some(Shape::Parallelogram),
            _ => None,
        };

        if selected_shape.is_none() {
            println!("Please input a number between 1 and 8\n")
        }
    }

    if let Some(shape) = selected_shape {
        shape.calculate();
    }
}
