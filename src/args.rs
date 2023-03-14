use clap::{command, Command};

pub fn get_app() -> Command {
    command!("ed")
        .subcommand(Command::new("bmi").about("Calculate BMI"))
        .subcommand(Command::new("bmr").about("Calculate BMR"))
        .subcommand(Command::new("tdee").about("Calculate TDEE"))
}
