use clap::Command;

pub fn get_app() -> Command {
    Command::new("ed")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(Command::new("bmi").about("Calculate BMI"))
        .subcommand(Command::new("bmr").about("Calculate BMR"))
        .subcommand(Command::new("tdee").about("Calculate TDEE"))
}
