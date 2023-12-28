use lib::read_number::read_number;
use std::f64::consts::PI;

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let radius = read_number("Radius:");

    let area = radius.powi(2) * PI;
    let perimeter = 2.0 * radius * PI;

    return (Some(area), Some(perimeter));
}
