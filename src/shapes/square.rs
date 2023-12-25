use lib::read_number::read_number;

pub fn calculate() -> f64 {
    let side = read_number("\"a\" side:");

    return side.powi(2);
}
