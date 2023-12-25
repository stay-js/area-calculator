use lib::read_number::read_number;

pub fn calculate() -> f64 {
    let base = read_number("Please provide the base");
    let height = read_number("Please provide the height");

    return base * height;
}
