use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

fn main() {
    let matches = command!()
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("name: {}", name);
    }
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("config: {}", config_path.display());
    }

    match matches
        .get_one::<u8>("debug")
        .expect("Counts are defaulted")
    {
        0 => println!("debug mode is off"),
        1 => println!("debug mode is kind of on"),
        2 => println!("debug mode is on"),
        _ => println!("don't be crazy"),
    }

    if let Some(list_matches) = matches.subcommand_matches("test") {
        if matches.get_flag("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}
