use lib::read_number::read_number;

pub fn calculate() -> f64 {
    let a = read_number("Please provide the \"a\" side");
    let b = read_number("Please provide the \"b\" side");

    return a * b;
}
