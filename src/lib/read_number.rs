use dialoguer::{theme::ColorfulTheme, Input};

pub fn read_number(prompt: &str) -> f64 {
    let mut number = -1.0;

    while number < 0.0 {
        number = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .validate_with(|input: &String| {
                return match input.parse::<f64>() {
                    Ok(num) => {
                        if num < 0.0 {
                            return Err("Must be bigger than 0");
                        }

                        return Ok(());
                    }
                    Err(_) => Err("Failed to parse input"),
                };
            })
            .interact_text()
            .expect("Failed to read input")
            .parse()
            .expect("Failed to parse input");
    }

    return number;
}
