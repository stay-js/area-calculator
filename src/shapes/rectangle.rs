use lib::read_number::read_number;

pub fn calculate() -> f64 {
    let a = read_number("\"a\" side:");
    let b = read_number("\"b\" side:");

    return a * b;
}
