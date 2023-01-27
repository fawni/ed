use bmr::Sex;
use dialoguer::{
    theme::{ColorfulTheme, SimpleTheme},
    Input, Select,
};
use owo_colors::OwoColorize;

mod bmi;
mod bmr;
mod tdee;

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
    // calculate_bmi()?;
    calculate_bmr()?;
    // calculate_tdee()?;

    Ok(())
}

fn calculate_bmi() -> Result<f32, Box<dyn std::error::Error>> {
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

    info!("BMI: {bmi:.1}");
    info!("New BMI: {new:.1}");
    info!("BMI Prime: {prime:.1}");

    Ok(bmi)
}

fn calculate_bmr() -> Result<(), Box<dyn std::error::Error>> {
    let weight: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your weight? (kg)")
        .interact_text()?;
    let height: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your height? (cm)")
        .interact()?;
    let age: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your age?")
        .validate_with(|a: &f32| -> Result<(), &str> {
            match a <= &125.0 {
                true => Ok(()),
                false => Err("Age must be less than 125"),
            }
        })
        .interact()?;
    let sex: Sex = Sex::from(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What is your sex?")
            .items(&[Sex::Male, Sex::Female])
            .interact()?,
    );

    let harris = bmr::harris_benedict::calculate(weight, height, age, sex);
    let harris_revised = bmr::harris_benedict::revised::calculate(weight, height, age, sex);

    println!("{}", "---".green());

    info!("Harris-Benedict (Original): {harris:.0}");
    info!("Harris-Benedict (Revised): {harris_revised:.0}");

    Ok(())
}

// fn calculate_tdee() -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }
