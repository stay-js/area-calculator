use lib::read_number::read_number;
use std::f64::consts::PI;

pub fn calculate() -> f64 {
    let semi_major_axis = read_number("Please provide the semi-major axis");
    let semi_minor_axis = read_number("Please provide the semi-minor axis");

    return PI * semi_major_axis * semi_minor_axis;
}
