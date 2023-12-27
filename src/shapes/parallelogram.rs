use lib::{
    read_number::read_number,
    to_calculate::{to_calculate, ToCalculate},
};

fn calculate_area() -> f64 {
    let base = read_number("Base:");
    let height = read_number("Height:");

    return base * height;
}

fn calculate_perimeter() -> f64 {
    let a = read_number("\"a\" side:");
    let b = read_number("\"b\" side:");

    return 2.0 * (a + b);
}

pub fn calculate() -> (Option<f64>, Option<f64>) {
    return match to_calculate() {
        ToCalculate::Area => (Some(calculate_area()), None),
        ToCalculate::Perimeter => (None, Some(calculate_perimeter())),
        ToCalculate::Both => (Some(calculate_area()), Some(calculate_perimeter())),
    };
}
