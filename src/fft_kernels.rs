extern crate argparse;
extern crate num;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use argparse::{ArgumentParser, StoreTrue, Store};

struct Arguments {
    verbose: bool,
    data_path: String,
    tol: f64,
    iter: usize,
    readonly: bool,
}

impl Arguments {
    // default values for arguments
    fn new() -> Arguments {
        Arguments { verbose: false, data_path: String::from(""), tol: 1e-5, iter: 1, readonly: false }
    }
}

struct Header {
    dummy: usize
}

fn read_header(namein: &str, data_path: &String) -> Header {
    let name = format!("{}/{}.bin",data_path,namein);
    let mut file = File::open(name).unwrap();

    Header {dummy: 0}
}

fn main() {
    let mut args = Arguments::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("SOCS standalone test");
        ap.refer(&mut args.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "be verbose");
        ap.refer(&mut args.data_path)
          .add_option(&["-d", "--data-path"], Store, "path to input data");
        ap.refer(&mut args.tol)
            .add_option(&["-t", "--tol"], Store,
            "set tolerance <eps fp value>");
        ap.refer(&mut args.iter)
            .add_option(&["-i", "--iter"], Store,
            "set # of iterations");
        ap.refer(&mut args.readonly)
            .add_option(&["-r", "--readonly"], StoreTrue,
            "set readonly");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }

    if args.verbose {
        println!("data path : {} number of iterations : {} tolerance : {:e} readonly : {}",
                 args.data_path, args.iter, args.tol, args.readonly);
    }

    let mut file = File::open("Cargo.toml").unwrap();
    let mut buf = [0u8; 12];
    file.read(&mut buf).unwrap();
    println!("{:?}", buf);
    // use file
}
