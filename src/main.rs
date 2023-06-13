use dialoguer::{theme::ColorfulTheme, Input, Select};

use bmr::Sex;
use tdee::ActivityLevel;

mod bmi;
mod bmr;
mod tdee;

mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match args::get_app().get_matches().subcommand() {
        Some(("bmi", _)) => calculate_bmi(),
        Some(("bmr", _)) => calculate_bmr(),
        Some(("tdee", _)) => calculate_tdee(),
        _ => {
            args::get_app().print_help()?;
            Ok(())
        }
    }
}

fn calculate_bmi() -> Result<(), Box<dyn std::error::Error>> {
    let weight: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your weight? (kg)")
        .interact_text()?;
    let height_cm: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your height? (cm)")
        .interact()?;

    let bmi = bmi::calculate(weight, height_cm);
    let new = bmi::new::calculate(weight, height_cm);
    let prime = bmi::prime::calculate(weight, height_cm);

    println!("\x1b[32m---\x1b[0m");

    twink::info!("BMI: {bmi:.1}");
    twink::info!("New BMI: {new:.1}");
    twink::info!("BMI Prime: {prime:.1}");

    Ok(())
}

fn calculate_bmr() -> Result<(), Box<dyn std::error::Error>> {
    let weight: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your weight? (kg)")
        .interact_text()?;
    let height: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your height? (cm)")
        .interact_text()?;
    let age: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your age?")
        .validate_with(|a: &f32| -> Result<(), &str> {
            match a <= &125.0 {
                true => Ok(()),
                false => Err("Age must be less than or equal to 125"),
            }
        })
        .interact_text()?;
    let sex: Sex = Sex::from(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What is your sex?")
            .items(&[Sex::Male, Sex::Female])
            .interact()?,
    );
    let _bf: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your body fat %? (optional)")
        .allow_empty(true)
        .show_default(false)
        .default(0.0)
        .interact_text()?;
    let bodyfat = if _bf == 0.0 { None } else { Some(_bf) };

    let harris = bmr::harris_benedict::calculate(weight, height, age, sex);
    let harris_revised = bmr::harris_benedict::revised::calculate(weight, height, age, sex);
    let mifflin = bmr::mifflin_st_jeor::calculate(weight, height, age, sex);

    println!("\x1b[32m---\x1b[0m");

    twink::info!("Harris-Benedict (Original): {harris:.0}");
    twink::info!("Harris-Benedict (Revised): {harris_revised:.0}");
    twink::info!("Mifflin St Jeor: {mifflin:.0}");

    if let Some(bf) = bodyfat {
        let katch = bmr::katch_mcardle::calculate(weight, bf);
        let katch_hyprid = bmr::katch_mcardle::hyprid::calculate(weight, bf);
        let cunningham = bmr::cunningham::calculate(weight, bf);

        twink::info!("Katch-McArdle: {katch:.0}");
        twink::info!("Katch-McArdle (Hyprid): {katch_hyprid:.0}");
        twink::info!("Cunningham: {cunningham:.0}");
    }

    Ok(())
}

fn calculate_tdee() -> Result<(), Box<dyn std::error::Error>> {
    let weight: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your weight? (kg)")
        .interact_text()?;
    let height: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your height? (cm)")
        .interact_text()?;
    let age: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your age?")
        .validate_with(|a: &f32| -> Result<(), &str> {
            if a <= &125.0 {
                Ok(())
            } else {
                Err("Age must be less than or equal to 125")
            }
        })
        .interact_text()?;
    let sex: Sex = Sex::from(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What is your sex?")
            .items(&[Sex::Male, Sex::Female])
            .interact()?,
    );
    let bf_: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your body fat %? (optional)")
        .allow_empty(true)
        .show_default(false)
        .default(0.0)
        .interact_text()?;
    let bodyfat = if bf_ == 0.0 { None } else { Some(bf_) };
    let activity_level = ActivityLevel::from(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What is your activity level?")
            .items(&[
                ActivityLevel::Paralyzed,
                ActivityLevel::Immobile,
                ActivityLevel::Constricted,
                ActivityLevel::Little,
                ActivityLevel::Sedentary,
                ActivityLevel::SlightlyActive,
                ActivityLevel::LightlyActive,
                ActivityLevel::ModeratelyActive,
                ActivityLevel::VeryActive,
                ActivityLevel::ExtremelyActive,
            ])
            .default(ActivityLevel::Sedentary as usize)
            .interact()?,
    );

    let harris = tdee::calculate(
        bmr::harris_benedict::calculate(weight, height, age, sex),
        activity_level,
    );
    let harris_revised = tdee::calculate(
        bmr::harris_benedict::revised::calculate(weight, height, age, sex),
        activity_level,
    );
    let mifflin = tdee::calculate(
        bmr::mifflin_st_jeor::calculate(weight, height, age, sex),
        activity_level,
    );

    println!("\x1b[32m---\x1b[0m");

    twink::info!("Harris-Benedict (Original): {harris:.0}");
    twink::info!("Harris-Benedict (Revised): {harris_revised:.0}");
    twink::info!("Mifflin St Jeor: {mifflin:.0}");

    if let Some(bf) = bodyfat {
        let katch = tdee::calculate(bmr::katch_mcardle::calculate(weight, bf), activity_level);
        let katch_hyprid = tdee::calculate(
            bmr::katch_mcardle::hyprid::calculate(weight, bf),
            activity_level,
        );
        let cunningham = tdee::calculate(bmr::cunningham::calculate(weight, bf), activity_level);

        twink::info!("Katch-McArdle: {katch:.0}");
        twink::info!("Katch-McArdle (Hyprid): {katch_hyprid:.0}");
        twink::info!("Cunningham: {cunningham:.0}");
    }

    Ok(())
}
