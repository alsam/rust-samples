use std::fs::File;
use std::io::{self, Read, BufRead};
use num_complex::Complex64;
use nalgebra_sparse::coo::CooMatrix;

#[derive(Clone, Debug)]
enum DataType {
    Real(Vec<f64>),
    Complex(Vec<Complex64>),
    Integer(Vec<isize>),
}

pub struct MatrixMarketReader {
    rows: usize,
    cols: usize,
    row: Vec<usize>,
    col: Vec<usize>,
    data: DataType,
}

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s).expect("file not found");
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| { line.split_whitespace().collect() }).collect()
}

impl MatrixMarketReader {
    pub fn new(fname: &str) -> Result<Self, String> {
        let whole_file = filename_to_string(fname).unwrap();
        let wbyl = &words_by_line(&whole_file);
        let mm_header = &wbyl[0];
        let (banner, mtx, coord, dtype, storage) = (mm_header[0],
                                                    mm_header[1],
                                                    mm_header[2],
                                                    mm_header[3],
                                                    mm_header[4]);
        if banner.ne("%%MatrixMarket") { return Err(String::from("no banner")); }
        if mtx.ne("matrix") { return Err(String::from("not a matrix")); }
        let is_symmetric = match storage {
                             "symmetric" => true,
                             "general"   => false,
                              _          => return Err(String::from("unsupported storage type"))};
        let is_sparse = match coord {
                          "coordinate"   => true,
                          "array"        => false,
                           _             => return Err(String::from("unsupported coordinate type"))};
        let mut data : DataType = match dtype {
                           "real"        => DataType::Real(Vec::new()),
                           "complex"     => DataType::Complex(Vec::new()),
                           "integer"     => DataType::Integer(Vec::new()),
                           _             => return Err(String::from("unsupported data type"))};

        let mut rows = 0usize;
        let mut cols = 0usize;
        let mut nnz  = 0usize;
        let mut row = Vec::<usize>::new();
        let mut col = Vec::<usize>::new();
        for words in wbyl {
            if words[0].starts_with('%') { // skip comments that starts with %
                //println!("words: {:?}", words);
            } else {
                // read triples, the 1st one contains (rows, cols, nnz) the others - (i, j, val)
                macro_rules! parse_word {
                    ($t:ty, $i:expr) => {
                        words[$i].parse::<$t>().unwrap()
                    };
                    ($i:expr) => { // `$t` defaulted to `i32`
                        parse_word!(usize, $i)
                    };
                }
                let reserve_data_vals = move |data: DataType, nnz: usize| match data {
                    DataType::Real(mut v)    => { v.reserve(nnz); DataType::Real(v)},
                    DataType::Integer(mut v) => { v.reserve(nnz); DataType::Integer(v)},
                    DataType::Complex(mut v) => { v.reserve(nnz); DataType::Complex(v)},
                };
                let parse_data_vals = move |data: DataType, i: usize| match data {
                    DataType::Real(mut v)    => { v.push(parse_word!(f64,   i));
                                                  DataType::Real(v)},
                    DataType::Integer(mut v) => { v.push(parse_word!(isize, i));
                                                  DataType::Integer(v)},
                    DataType::Complex(mut v) => { v.push(Complex64::new(parse_word!(f64, i),
                                                                        parse_word!(f64, i + 1)));
                                                  DataType::Complex(v)},
                };
                if rows == 0 {
                    rows = parse_word!(0);
                    cols = parse_word!(1);
                    nnz  = parse_word!(2);
                    if is_symmetric { nnz *= 2; }
                    row.reserve(nnz);
                    col.reserve(nnz);
                    data = reserve_data_vals(data, nnz);
                    //println!("rows: {} cols: {} nnz: {}", rows, cols, nnz);
                } else {
                    let x = parse_word!(0);
                    let y = parse_word!(1);
                    row.push(x);
                    col.push(y);
                    data = parse_data_vals(data, 2);
                    //println!("x: {} y: {} v: {:10.4e}", x, y, v);
                }
            }
            //println!("words: {:?}", words);
        }

        println!("wbyl[0]: {:?}", mm_header);
        //println!("data: {:?}", data);
        Ok(Self { rows: rows, cols: cols, row: row, col: col, data: data, })
    }
}
 
fn use_it(fname: &str) {
    let mm = MatrixMarketReader::new("xxx");
}
 
