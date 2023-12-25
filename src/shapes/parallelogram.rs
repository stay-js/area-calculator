use lib::read_number::read_number;

pub fn calculate() -> f64 {
    let base = read_number("Base");
    let height = read_number("Height");

    return base * height;
}
