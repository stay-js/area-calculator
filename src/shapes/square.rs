use lib::read_number::read_number;

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let side = read_number("\"a\" side:");

    let area = side.powi(2);
    let perimeter = 4.0 * side;

    return (Some(area), Some(perimeter));
}
