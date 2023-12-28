use lib::read_number::read_number;
use std::f64::consts::PI;

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let semi_major_axis = read_number("Semi-major axis:");
    let semi_minor_axis = read_number("Semi-minor axis:");

    let area = semi_major_axis * semi_minor_axis * PI;

    return (Some(area), None);
}
