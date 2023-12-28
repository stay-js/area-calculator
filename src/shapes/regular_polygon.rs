use lib::read_number::read_number;
use std::f64::consts::PI;

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let number_of_sides = read_number("Number of sides:");
    let side = read_number("Side:");

    let area = number_of_sides / 4.0 * side.powi(2) / (PI / number_of_sides).tan();
    let perimeter = number_of_sides * side;

    return (Some(area), Some(perimeter));
}
