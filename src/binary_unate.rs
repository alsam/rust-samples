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
    struct BinateVarAttrs(isize, isize, isize); // True, Complement, Index

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

    fn cmp_binate(left: &BinateVarAttrs, right: &BinateVarAttrs) -> Ordering {
        match (left, right) {
            (&BinateVarAttrs(t1, c1, i1), &BinateVarAttrs(t2, c2, i2)) =>
                if      t2+c2 < t1+c1 { Ordering::Less }
                else if t2+c2 > t1+c1 { Ordering::Greater }
                else { // t1+c1 == t2+c2
                    if      (t1-c1).abs() < (t1-c1).abs() { Ordering::Less }
                    else if (t1-c1).abs() > (t1-c1).abs() { Ordering::Greater }
                    else if i1 < i2 { Ordering::Less }
                    else { Ordering::Equal }
                }
        
        }

    }

    fn cmp_unate(left: &BinateVarAttrs, right: &BinateVarAttrs) -> Ordering {
        match (left, right) {
            (&BinateVarAttrs(t1, c1, i1), &BinateVarAttrs(t2, c2, i2)) =>
                if      t2+c2 < t1+c1 { Ordering::Less }
                else if t2+c2 > t1+c1 { Ordering::Greater }
                else { // t1+c1 == t2+c2
                    if i1 < i2 { Ordering::Less }
                    else { Ordering::Equal }
                }
        
        }

    }

    let num_cubes = F.len();
    let num_vars = if num_cubes >= 1 { F.front().unwrap().len() } else { 0 };
    let mut tie = vec![BinateVarAttrs(0, 0, 0); num_vars];
    for c in F.iter() {
        for i in 0 .. (*c).len() {
            let term = (*c)[i].clone();
            let j = i as isize;
            if        term == TriLogic::True {
                tie[i] = match tie[i] { BinateVarAttrs(t,c,_) => BinateVarAttrs(t+1,c,j) }
            } else if term == TriLogic::False {
                tie[i] = match tie[i] { BinateVarAttrs(t,c,_) => BinateVarAttrs(t,c+1,j) }
            } else {
                tie[i] = match tie[i] { BinateVarAttrs(t,c,_) => BinateVarAttrs(t,c,j) }
            }
        }
    }

    for k in 0..num_vars {
        let BinateVarAttrs(t,c,i) = tie[k];
        println!("tie[{}] = ({},{},{})",k,t,c,i);
    }

    let there_are_binate_vars = tie.iter()
        .any(|x| match *x { BinateVarAttrs(t,c,_) => t > 0 && c > 0 } );
    if there_are_binate_vars {
        // filter tie, leave only binate cubes
        let tie_binate: Vec<BinateVarAttrs> = tie.iter()
            .filter(|&x| match *x { BinateVarAttrs(t,c,_)  => t > 0 && c > 0  } )
            .map(|x| x.clone()).collect();
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
    let is = File::open(inp_fname).unwrap(); // TODO try with `try!`
    let reader = BufReader::new(is);
    let mut line_num = 0;
    let mut num_vars:  usize = 0;
    let mut num_cubes: usize = 0;
    let mut cube_list: Vec<Vec<TriLogic>> = Vec::new();

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        line_num += 1;
        println!("<{}> : {}", line_num, line);
        let chunks: Vec<&str> = line.split_terminator(|c: char| c.is_whitespace()).collect();
        match line_num {
            1 => num_vars  = chunks[0].parse().ok().expect("a number expected"),
            2 =>
            {
                num_cubes  = chunks[0].parse().ok().expect("a number expected");
                println!("num_vars: {} num_cubes: {}", num_vars, num_cubes);
                cube_list.resize(num_cubes, vec![TriLogic::DontCare; num_vars]);
            },
            _ =>
            {
                if (line_num - 2) > num_cubes { break }
                let nterms: usize  = chunks[0].parse().ok().expect("a number expected");
                let terms: Vec<isize> = Vec::from_iter ((0..nterms)
                                            .map (|idx| chunks[idx+1].parse().ok()
                                                  .expect("a number expected")));
                println!("nterms: {} terms: {:?}", nterms, terms);
            }
        }

    }
}
