use lib::{
    read_number::read_number,
    to_calculate::{to_calculate, ToCalculate},
};

pub fn calculate_area() -> (Option<f64>, Option<f64>) {
    let base_1 = read_number("Base 1:");
    let base_2 = read_number("Base 2:");
    let height = read_number("Height:");

    let area = (base_1 + base_2) / 2.0 * height;

    return (Some(area), None);
}

pub fn calculate_perimeter() -> (Option<f64>, Option<f64>) {
    let a = read_number("\"a\" side:");
    let b = read_number("\"b\" side:");
    let c = read_number("\"c\" side:");
    let d = read_number("\"d\" side:");

    let perimeter = a + b + c + d;

    return (None, Some(perimeter));
}

pub fn calculate_both() -> (Option<f64>, Option<f64>) {
    let a = read_number("\"a\" side (base 1):");
    let c = read_number("\"c\" side (base 2):");
    let b = read_number("\"b\" side:");
    let d = read_number("\"d\" side:");
    let height = read_number("Height:");

    let area = (a + c) / 2.0 * height;
    let perimeter = a + b + c + d;

    return (Some(area), Some(perimeter));
}

pub fn calculate() -> (Option<f64>, Option<f64>) {
    return match to_calculate() {
        ToCalculate::Area => calculate_area(),
        ToCalculate::Perimeter => calculate_perimeter(),
        ToCalculate::Both => calculate_both(),
    };
}
