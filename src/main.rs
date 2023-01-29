use dialoguer::{theme::ColorfulTheme, Input, Select};
use owo_colors::OwoColorize;

use bmr::Sex;
use tdee::ActivityLevel;

mod bmi;
mod bmr;
mod tdee;

mod args;
mod macros;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match args::get_app().get_matches().subcommand() {
        Some(("bmi", _)) => calculate_bmi(),
        Some(("bmr", _)) => calculate_bmr(),
        Some(("tdee", _)) => calculate_tdee(),
        None => {
            args::get_app().print_help()?;
            Ok(())
        }
        _ => unreachable!(),
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

    println!("{}", "---".green());

    info!("BMI: {bmi:.1}");
    info!("New BMI: {new:.1}");
    info!("BMI Prime: {prime:.1}");

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
        .with_prompt("What is your bodyfat%? (optional)")
        .allow_empty(true)
        .show_default(false)
        .default(0.0)
        .interact_text()?;
    let bodyfat = if _bf == 0.0 { None } else { Some(_bf) };

    let harris = bmr::harris_benedict::calculate(weight, height, age, sex);
    let harris_revised = bmr::harris_benedict::revised::calculate(weight, height, age, sex);
    let mifflin = bmr::mifflin_st_jeor::calculate(weight, height, age, sex);

    println!("{}", "---".green());

    info!("Harris-Benedict (Original): {harris:.0}");
    info!("Harris-Benedict (Revised): {harris_revised:.0}");
    info!("Mifflin St Jeor: {mifflin:.0}");

    if let Some(bf) = bodyfat {
        let katch = bmr::katch_mcardle::calculate(weight, bf);
        let katch_hyprid = bmr::katch_mcardle::hyprid::calculate(weight, bf);
        let cunningham = bmr::cunningham::calculate(weight, bf);

        info!("Katch-McArdle: {katch:.0}");
        info!("Katch-McArdle (Hyprid): {katch_hyprid:.0}");
        info!("Cunningham: {cunningham:.0}");
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

    println!("{}", "---".green());

    info!("Harris-Benedict (Original): {harris:.0}");
    info!("Harris-Benedict (Revised): {harris_revised:.0}");
    info!("Mifflin St Jeor: {mifflin:.0}");

    if let Some(bf) = bodyfat {
        let katch = tdee::calculate(bmr::katch_mcardle::calculate(weight, bf), activity_level);
        let katch_hyprid = tdee::calculate(
            bmr::katch_mcardle::hyprid::calculate(weight, bf),
            activity_level,
        );
        let cunningham = tdee::calculate(bmr::cunningham::calculate(weight, bf), activity_level);

        info!("Katch-McArdle: {katch:.0}");
        info!("Katch-McArdle (Hyprid): {katch_hyprid:.0}");
        info!("Cunningham: {cunningham:.0}");
    }

    Ok(())
}
