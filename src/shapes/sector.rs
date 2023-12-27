use lib::read_number::read_number;
use std::f64::consts::PI;

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let radius = read_number("Radius:");
    let angle = read_number("Angle:");

    let area = PI * radius.powi(2) * angle / 360.0;
    let perimeter = 2.0 * PI * radius * angle / 360.0;

    return (Some(area), Some(perimeter));
}
