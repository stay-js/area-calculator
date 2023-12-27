use dialoguer::{theme::ColorfulTheme, Select};

pub enum ToCalculate {
    Area,
    Perimeter,
    Both,
}

pub fn to_calculate() -> ToCalculate {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("To calculate:")
        .items(&["Area", "Perimeter", "Both"])
        .interact()
        .expect("Failed to select to calculate");

    let to_calculate = match selection {
        0 => Some(ToCalculate::Area),
        1 => Some(ToCalculate::Perimeter),
        2 => Some(ToCalculate::Both),
        _ => None,
    };

    return to_calculate.unwrap();
}
