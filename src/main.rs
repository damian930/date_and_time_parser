use anyhow::Result;
use damians_custom_datetime_parser::*;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "help" => get_help(),
            "credits" => print_credits(),
            _ => print!("invlaid command"),
        }

        return Ok(());
    }

    if args.len() == 3 {
        match (args[1].as_str(), args[2].as_str()) {
            ("parse", date_time) => println!("{:#?}", parse(date_time)),
            (_, _) => print!("invlaid command"),
        }

        return Ok(());
    }

    Ok(())
}

// ================================================
// CMD operations

fn parse(date_time: &str) -> Result<DateTime, DateTimeError> {
    DateTime::from_data_time(date_time)
}

fn get_help() {
    println!("DateTime Parser CLI Usage: ");
    println!("    cargo run parse \"<DateTime>\": Parses given DateTime and prints it`s parts.");
    println!("    cargo run credits: Prints projectc credits.");
    println!("    cargo run help: Prints help.");
}

fn print_credits() {
    println!("DateTime Parser, created by Damian Dragovoz.");
}
