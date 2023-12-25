use lib::read_number::read_number;

pub fn calculate() -> f64 {
    let base_1 = read_number("Base 1");
    let base_2 = read_number("Base 2");
    let height = read_number("Height");

    return ((base_1 + base_2) / 2.0) * height;
}
