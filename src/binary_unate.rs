// program args
use std::env;

// io/fs ops
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{File, OpenOptions};
use std::path::Path;

// a double linked list
use std::collections::LinkedList;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum TriLogic {
    True      = 0b01,
    False     = 0b10,
    DontCare  = 0b11
}

type CubeList =  LinkedList<[TriLogic]>;

//fn cubelist_contains_all_dont_cares_cube(F: &CubeList) -> bool {
//    // `any` of the cubes contains `all` dont cares
//    F.iter().any(|c: &[TriLogic]| ((*c).iter().all(|t: &TriLogic| *t == TriLogic::DontCare )))
//}


fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("plz. gimme input and output fnames");
        return;
    }

    let inp_fname = args[1].clone();
    let out_fname = args[2].clone();
    let mut is = File::open(Path::new(&inp_fname)).unwrap(); // try!() macro doesn't work
    let mut reader = BufReader::new(is);

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        println!("line : {}", line);

    }
}
