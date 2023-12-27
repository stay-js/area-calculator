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

    return match selection {
        0 => ToCalculate::Area,
        1 => ToCalculate::Perimeter,
        2 => ToCalculate::Both,
        _ => panic!("Invalid selection"),
    };
}
