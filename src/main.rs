use dialoguer::{theme::ColorfulTheme, Input};
use owo_colors::OwoColorize;

mod bmi;
// mod bmr;
// mod tdee;

macro_rules! info {
    ($($arg:tt)*) => {
        println!(
            "{}{}{} {}",
            "[".bright_black(),
            "+".green(),
            "]".bright_black(),
            format!($($arg)*))
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weight: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your weight? (kg)")
        .interact_text()?;
    let height_cm: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your height? (cm)")
        .interact()?;

    let bmi = bmi::calculate(weight, height_cm);
    let new = bmi::new::calculate(weight, height_cm);
    let prime = bmi::prime::calculate(weight, height_cm);

    println!("{}", "---".green());

    info!("BMI: {bmi:.3}");
    info!("New BMI: {new:.3}");
    info!("BMI Prime: {prime:.1}");

    Ok(())
}
