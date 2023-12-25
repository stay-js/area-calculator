use lib::read_number::read_number;
use std::f64::consts::PI;

pub fn calculate() -> f64 {
    let radius = read_number("Please provide the radius");
    let angle = read_number("Please provide the angle");

    return (PI * radius * radius) * (angle / 360.0);
}
