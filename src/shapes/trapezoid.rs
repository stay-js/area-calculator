use lib::read_number::read_number;

pub fn calculate() -> f64 {
    let base_1 = read_number("Please provide base 1");
    let base_2 = read_number("Please provide base 2");
    let height = read_number("Please provide the height");

    return ((base_1 + base_2) / 2.0) * height;
}
