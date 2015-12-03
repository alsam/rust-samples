extern crate argparse;
extern crate num;
extern crate byteorder;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::mem;
use byteorder::{ByteOrder, LittleEndian, BigEndian, NativeEndian};
use std::process::exit;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::f64::consts::PI;
use std::iter::FromIterator;

type c32 = num::Complex<f32>;

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
        Arguments {
            verbose: false,
            data_path: String::from(""),
            tol: 1e-5,
            iter: 1,
            readonly: false,
        }
    }
}

struct Header {
    endianness: u8,
    grid_size: (usize, usize),
    grid: Vec<Vec<c32>>,
}

impl Header {
    fn new(e: u8, x: usize, y: usize, read_buf: &Vec<u8>) -> Header {
        let xsize = x;
        let ysize = y;
        Header {
            endianness: e,
            grid_size: (x, y),
            grid: Vec::from_iter((0..xsize)
                         .map (|i|
                  Vec::from_iter((0..ysize)
                         .map (|j| {
                             let start = (i*ysize+j) * 2 * mem::size_of::<f32>();
                             let (real, imag) = if e == 0 {
                                 (LittleEndian::read_f32(&read_buf[start + 0 .. start + 4]),
                                  LittleEndian::read_f32(&read_buf[start + 4 .. start + 8]) )
                                                } else {
                                 (BigEndian::read_f32(&read_buf[start + 0 .. start + 4]),
                                  BigEndian::read_f32(&read_buf[start + 4 .. start + 8]) )};
                             //println!("start: {} buf slice1: {:?} slice2: {:?} real: {} imag: {}",
                             //          start, &read_buf[start + 0 .. start + 4], &read_buf[start + 4 .. start + 8], real, imag);
                             c32::new(real, imag)
                          } ) ) ) ),

        }
    }
}

fn read_header(namein: &str, data_path: &String) -> Header {
    fn is_big_endian() -> bool {
        let probe = [43u8, 21];
        probe[0] == 21u8
    }

    let name = format!("{}/{}.bin", data_path, namein);
    let mut file = File::open(name).unwrap();
    let mut buf = [0u8; 8];
    file.read(&mut buf[0..1]).unwrap();
    unsafe {
        let endianness = mem::transmute::<[u8; 1], u8>([buf[0]]);
        let (xe, ye) = if endianness == 0 {
            file.read(&mut buf[0..4]).unwrap();
            let xe = LittleEndian::read_u32(&buf[0..4]);
            file.read(&mut buf[0..4]).unwrap();
            let ye = LittleEndian::read_u32(&buf[0..4]);

            (xe, ye)

        } else {
            file.read(&mut buf[0..4]).unwrap();
            let xe = BigEndian::read_u32(&buf[0..4]);
            file.read(&mut buf[0..4]).unwrap();
            let ye = BigEndian::read_u32(&buf[0..4]);

            (xe, ye)
        };

        let grid_sz = (xe * ye) as usize;
        let grid_elem_size = mem::size_of::<c32>();
        let mut read_buf = Vec::new();
        let sz = file.read(&mut read_buf).unwrap();
        if sz != (grid_sz * grid_elem_size) {
            panic!("corrupted grid sz: {} xe*ye: {}", sz, xe * ye);
        }

        Header::new(endianness, xe as usize, ye as usize, &read_buf)
    }

}

fn main() {
    let mut args = Arguments::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("SOCS standalone test");
        ap.refer(&mut args.verbose)
          .add_option(&["-v", "--verbose"], StoreTrue, "be verbose");
        ap.refer(&mut args.data_path)
          .add_option(&["-d", "--data-path"], Store, "path to input data");
        ap.refer(&mut args.tol)
          .add_option(&["-t", "--tol"], Store, "set tolerance <eps fp value>");
        ap.refer(&mut args.iter)
          .add_option(&["-i", "--iter"], Store, "set # of iterations");
        ap.refer(&mut args.readonly)
          .add_option(&["-r", "--readonly"], StoreTrue, "set readonly");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }

    if args.verbose {
        println!("data path : {} number of iterations : {} tolerance : {:e} readonly : {}",
                 args.data_path,
                 args.iter,
                 args.tol,
                 args.readonly);
    }

    let mut file = File::open("Cargo.toml").unwrap();
    let mut buf = [0u8; 12];
    file.read(&mut buf).unwrap();
    println!("{:?}", buf);
    // use file
}
