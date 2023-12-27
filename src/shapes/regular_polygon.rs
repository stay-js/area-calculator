use lib::read_number::read_number;
use std::f64::consts::PI;

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let number_of_sides = read_number("Number of sides:");
    let side = read_number("Side:");

    let area = 1.0 / 4.0 * number_of_sides * side.powi(2) * 1.0 / (PI / number_of_sides).tan();
    let perimeter = number_of_sides * side;

    return (Some(area), Some(perimeter));
}
