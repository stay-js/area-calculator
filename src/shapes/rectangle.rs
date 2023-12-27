use lib::read_number::read_number;

pub fn calculate() -> (Option<f64>, Option<f64>) {
    let a = read_number("\"a\" side:");
    let b = read_number("\"b\" side:");

    let area = a * b;
    let perimeter = 2.0 * (a + b);

    return (Some(area), Some(perimeter));
}
