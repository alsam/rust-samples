extern crate argparse;
extern crate num;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut buf = [0u8; 12];
    file.read(&mut buf).unwrap();
    println!("{:?}", buf);
    // use file
}
