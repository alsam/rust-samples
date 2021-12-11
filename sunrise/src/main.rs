extern crate sunrise;
#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};
//use chrono::{NaiveTime, NaiveDateTime};
extern crate chrono;

use chrono::prelude::*;

fn main() {
    let matches = App::new("sunrise-exe")
                          .version("1.0")
                          .about("prints out sunrise and sunset time")
                          .args_from_usage(
                              "-c, --config=[FILE] 'Sets a custom config file'
                              <INPUT>              'Sets the input file to use'
                              -v...                'Sets the level of verbosity'")
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg_from_usage("-d, --debug 'Print debug information'"))
                          .get_matches();


    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // Calculate times for December 11, 2021 in Moscow
    let (sunrise, sunset) = sunrise::sunrise_sunset(
        55.4544,
        37.3763,
        2021,
        12,
        11,
    );
    println!("sunrise: {} sunset: {}", sunrise, sunset);

    // https://stackoverflow.com/questions/50072055/converting-unix-timestamp-to-readable-time-string-in-rust
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(sunrise, 0);
    
    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    
    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    // Print the newly formatted date and time
    println!("{}", newdate);
}
