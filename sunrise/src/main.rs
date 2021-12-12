extern crate sunrise;
#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate chrono;
use chrono::prelude::{DateTime};
use chrono::{Utc, Local, NaiveDate, Datelike};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::convert::TryInto;

fn timestamp_to_time(timestamp: i64) -> String {
    // https://stackoverflow.com/questions/50072055/converting-unix-timestamp-to-readable-time-string-in-rust

    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(timestamp.try_into().unwrap());
    // Create DateTime from SystemTime
    //let datetime = DateTime::<Utc>::from(d);
    let datetime = DateTime::<Local>::from(d);
    // Formats the combined date and time with the specified format string.
    //let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    let timestamp_str = datetime.format("%H:%M:%S").to_string();
    timestamp_str
}

fn main() {
    let matches = App::new("sunrise-exe")
                          .version("1.0")
                          .about("prints out sunrise and sunset time")
                          .args_from_usage(
                              "-c, --config=[FILE]        'Sets a custom config file'
                              -l, --longitude=[LONGITUDE] 'Sets the place longitude'
                              -t, --latitude=[LATITUDE]   'Sets the place latitude'
                              -d, --date=[DATE]           'Sets the date in local timezone'
                              -v...                       'Sets the level of verbosity'")
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg_from_usage("-d, --debug 'Print debug information'"))
                          .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    let verbosity_level = matches.occurrences_of("v");

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    if verbosity_level >= 2 {
        println!("Value for config: {}", config);
    }

    let longitude_str = matches.value_of("longitude").unwrap_or("55.4544");
    let latitude_str = matches.value_of("latitude").unwrap_or("37.3763");

    let longitude = longitude_str.parse::<f64>().unwrap();
    let latitude = latitude_str.parse::<f64>().unwrap();
    if verbosity_level >= 2 {
        println!("longitude {} latitude {}", longitude, latitude);
    }

    // https://stackoverflow.com/questions/61179070/rust-chrono-parse-date-string-parseerrornotenough-and-parseerrortooshort
    let date_str = matches.value_of("date").unwrap_or("1970-01-01");
    if verbosity_level >= 3 {
        println!("Value for date: {}", date_str);
    }
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    if verbosity_level >= 1 {
        println!("Parsed date: {:?} {} {} {}",
            &naive_date, naive_date.year(), naive_date.month(), naive_date.day());
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
        longitude,          // 55.4544,
        latitude,           // 37.3763,
        naive_date.year(),  // 2021,
        naive_date.month(), // 12,
        naive_date.day(),   // 11,
    );
    if verbosity_level >= 3 {
        println!("sunrise: {} sunset: {}", sunrise, sunset);
    }

    let sunrise_str = timestamp_to_time(sunrise);
    let sunset_str = timestamp_to_time(sunset);

    println!{"sunrize {} and sunset {} at {:?} for ({} {})",
        sunrise_str, sunset_str, &naive_date, longitude, latitude};

}
