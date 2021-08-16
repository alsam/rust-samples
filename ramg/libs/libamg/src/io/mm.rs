use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use num_complex::Complex64;
use nalgebra_sparse::coo::CooMatrix;

enum DataType {
    Real(Vec<f64>),
    Complex(Vec<Complex64>),
    Integer(Vec<isize>),
}

pub struct MatrixMarketReader {
    rows: usize,
    cols: usize,
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
        //let file = File::open(fname).expect("file not found!");
        //let mut f = BufReader::new(file);
        let whole_file = filename_to_string(fname).unwrap();
        let wbyl = &words_by_line(&whole_file);
        let mm_header = &wbyl[0];
        let (banner, mtx, coord, dtype, storage) = (mm_header[0],
                                                    mm_header[1],
                                                    mm_header[2],
                                                    mm_header[3],
                                                    mm_header[4]);
        //if banner.ne("%%MatrixMarket") { panic!("no banner"); }
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
        let data = match dtype {
                       "real"        => DataType::Real(Vec::new()),
                       "complex"     => DataType::Complex(Vec::new()),
                       "integer"     => DataType::Integer(Vec::new()),
                       _             => return Err(String::from("unsupported data type"))};

        //let mut (rows, cols, nnz) = (0, 0, 0)
        let mut rows = 0;
        let mut cols = 0;
        let mut nnz  = 0;
        let mut row = Vec::<usize>::new();
        let mut col = Vec::<usize>::new();
        for words in wbyl {
            if words[0].starts_with('%') { // skip comments that starts with %
                //println!("words: {:?}", words);
            } else {
                // read triples, the 1st one contains (rows, cols, nnz) the others - (i, j, val)
                //let parse_word = |i: usize| words[i].parse().unwrap();
                macro_rules! parse_word {
                    ($t:ty, $i:expr) => {
                        words[$i].parse::<$t>().unwrap()
                    };
                    ($i:expr) => { // `$t` defaulted to `i32`
                        parse_word!(i32, $i)
                    };
                }
                if rows == 0 {
                    rows = parse_word!(0);
                    cols = parse_word!(1);
                    nnz  = parse_word!(2);
                    if is_symmetric { nnz *= 2; }



                    //println!("rows: {} cols: {} nnz: {}", rows, cols, nnz);
                } else {
                    let x: i32 = parse_word!(0);
                    let y: i32 = parse_word!(1);
                    let v: f64 = parse_word!(f64, 2);
                    //println!("x: {} y: {} v: {:10.4e}", x, y, v);
                }
            }
            //println!("words: {:?}", words);
        }

        println!("wbyl[0]: {:?}", mm_header);
        Ok(Self { rows: rows as usize, cols: cols as usize, data: data, })
    }
}
 
fn use_it(fname: &str) {
    let mm = MatrixMarketReader::new("xxx");
}
 
