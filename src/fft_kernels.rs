extern crate argparse;
extern crate num;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use argparse::{ArgumentParser, StoreTrue, Store};

struct Arguments {
    verbose: bool,
    tol: f64,
    iter: usize,
    readonly: bool,
}

impl Arguments {
    // default values for arguments
    fn new() -> Arguments {
        Arguments { verbose: false, tol: 1e-5, iter: 1, readonly: false }
    }
}

fn main() {
    let mut arguments = Arguments::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("SOCS standalone test");
        ap.refer(&mut arguments.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "be verbose");
        ap.refer(&mut arguments.tol)
            .add_option(&["-t", "--tol"], Store,
            "set tolerance <eps fp value>");
        ap.refer(&mut arguments.iter)
            .add_option(&["-i", "--iter"], Store,
            "set # of iterations");
        ap.refer(&mut arguments.readonly)
            .add_option(&["-r", "--readonly"], StoreTrue,
            "set readonly");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }

    if arguments.verbose {
        println!("number of iterations : {} tolerance : {:e} readonly : {}",
                 arguments.iter, arguments.tol, arguments.readonly);
    }

    let mut file = File::open("Cargo.toml").unwrap();
    let mut buf = [0u8; 12];
    file.read(&mut buf).unwrap();
    println!("{:?}", buf);
    // use file
}
