#[macro_use]
extern crate clap;

extern crate libamg;
extern crate nalgebra as na;

use libamg::io::MatrixMarketReader;
use std::time::{Duration, Instant};
use na::io::cs_matrix_from_matrix_market;

fn main()
{
    let matches = clap_app!(ramg =>
            (version: "0.0.1")
            (author: "Alexander Samoilov <alexander.samoilov@gmail.com>")
            (@arg SET_MATRIX: -A --matrix +takes_value "System matrix in the MatrixMarket format.")
            (@arg SET_BLOCKSIZE: -b --block-size +takes_value "The block size of the system matrix.")
        ).get_matches();

    let block_size = if let Ok(block_size) = value_t!(matches, "SET_BLOCKSIZE", usize) {
            block_size
        } else {
            1 // default value
        };
    println!("block size: {}", block_size);

    if let Ok(matrix_name) = value_t!(matches, "SET_MATRIX", String) {
        println!("the matrix: {}", matrix_name);
        let mut start = Instant::now();
        let mmr = MatrixMarketReader::new(&matrix_name);
        let mut duration = start.elapsed();
        println!("Time elapsed in `MatrixMarketReader::new()` is: {:?}", duration);
        start = Instant::now();
        let csr = libamg::io::mm::create_csr(&mmr.unwrap());
        duration = start.elapsed();
        println!("Time elapsed in `create_csr()` is: {:?}", duration);
        //println!("csr: {:?}", &csr);

        start = Instant::now();
        let cs = cs_matrix_from_matrix_market::<f64, &str>(&matrix_name);
        duration = start.elapsed();
        println!("Time elapsed in `cs_matrix_from_matrix_market()` is: {:?}", duration);
        //println!("cs: {:?}", &cs);

    }
}
