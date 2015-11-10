// program args
use std::env;

// io/fs ops
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{File, OpenOptions};
use std::path::Path;

// a double linked list
use std::collections::LinkedList;
use std::iter::FromIterator;

// for Ord
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum TriLogic {
    True      = 0b01,
    False     = 0b10,
    DontCare  = 0b11
}

type CubeList =  LinkedList<Vec<TriLogic>>;

fn cubelist_contains_all_dont_cares_cube(F: &CubeList) -> bool {
    // `any` of the cubes contains `all` dont cares
    F.iter().any(|c: &Vec<TriLogic>| ((*c).iter().all(|t: &TriLogic| *t == TriLogic::DontCare )))
}

#[test]
fn test_contains_all_dont_cares_cube() {
    assert!(!cubelist_contains_all_dont_cares_cube(
            &LinkedList::from_iter(vec![])),
            "case0: empty cubelist");
    assert!(!cubelist_contains_all_dont_cares_cube(
            &LinkedList::from_iter(vec![vec![TriLogic::True, TriLogic::DontCare, TriLogic::DontCare]])),
            "case1: [[True, DontCare, DontCare]]");
    assert!( cubelist_contains_all_dont_cares_cube(
            &LinkedList::from_iter(vec![vec![TriLogic::DontCare, TriLogic::DontCare, TriLogic::DontCare],
                                        vec![TriLogic::True,     TriLogic::False,    TriLogic::DontCare]])),
            "case2: [[DontCare, ...]]");
}

fn count_all_dont_cares(cube: &[TriLogic]) -> usize {
    (*cube).iter().filter(|term| **term == TriLogic::DontCare).count()
}

fn count_all_not_dont_cares(cube: &[TriLogic]) -> usize {
    (*cube).iter().filter(|term| **term != TriLogic::DontCare).count()
}

#[test]
fn test_count_don_cares() {
    let cube = [TriLogic::True,     TriLogic::False,
                TriLogic::DontCare, TriLogic::DontCare,
                TriLogic::False,    TriLogic::DontCare, TriLogic::True];
    assert!(count_all_dont_cares(&cube) == 3, "count all dont_cares");
    assert!(count_all_not_dont_cares(&cube) == 4, "count all not dont_cares");
}
fn most_binate_variable(F: &CubeList) -> usize {

    #[derive(Clone, PartialEq, PartialOrd, Eq)]
    struct BinateVarAttrs(i32, i32, i32); // True, Complement, Index

    impl std::cmp::Ord for BinateVarAttrs {
        #[inline]
        fn cmp(&self, other: &BinateVarAttrs) -> Ordering {
            match (self, other) {
                (&BinateVarAttrs(t1, c1, i1), &BinateVarAttrs(t2, c2, i2)) =>
                    if      t2+c2 < t1+c1 { Ordering::Less }
                    else if t2+c2 > t1+c1 { Ordering::Greater }
                    else { // t1+c1 == t2+c2
                        if      (t1 - c1).abs() < (t2 - c2).abs() { Ordering::Less }
                        else if (t1 - c1).abs() > (t2 - c2).abs() { Ordering::Greater }
                        else if i1 < i2 { Ordering::Less }
                        else { Ordering::Equal }
                    }

            }

        }

    }
    0 as usize
}

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
