// The MIT License (MIT)
//
// Copyright (c) 2015 Alexander Samoilov
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


// program args
use std::env;

// io/fs ops
use std::io;
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
    True = 0b01,
    False = 0b10,
    DontCare = 0b11,
}

type CubeList = LinkedList<Vec<TriLogic>>;

fn cubelist_contains_all_dont_cares_cube(F: &CubeList) -> bool {
    // `any` of the cubes contains `all` dont cares
    F.iter().any(|c: &Vec<TriLogic>| {
        ((*c).iter().all(|t: &TriLogic| *t == TriLogic::DontCare))
    })
}

#[test]
fn test_contains_all_dont_cares_cube() {
    assert!(
        !cubelist_contains_all_dont_cares_cube(&LinkedList::from_iter(vec![])),
        "case0: empty cubelist"
    );
    assert!(
        !cubelist_contains_all_dont_cares_cube(&LinkedList::from_iter(vec![
            vec![
                TriLogic::True,
                TriLogic::DontCare,
                TriLogic::DontCare,
            ],
        ])),
        "case1: [[True, DontCare, DontCare]]"
    );
    assert!(
        cubelist_contains_all_dont_cares_cube(&LinkedList::from_iter(vec![
            vec![
                TriLogic::DontCare,
                TriLogic::DontCare,
                TriLogic::DontCare,
            ],
            vec![
                TriLogic::True,
                TriLogic::False,
                TriLogic::DontCare,
            ],
        ])),
        "case2: [[DontCare, ...]]"
    );
}

fn count_all_dont_cares(cube: &[TriLogic]) -> usize {
    (*cube)
        .iter()
        .filter(|term| **term == TriLogic::DontCare)
        .count()
}

fn count_all_not_dont_cares(cube: &[TriLogic]) -> usize {
    (*cube)
        .iter()
        .filter(|term| **term != TriLogic::DontCare)
        .count()
}

#[test]
fn test_count_don_cares() {
    let cube = [
        TriLogic::True,
        TriLogic::False,
        TriLogic::DontCare,
        TriLogic::DontCare,
        TriLogic::False,
        TriLogic::DontCare,
        TriLogic::True,
    ];
    assert!(count_all_dont_cares(&cube) == 3, "count all dont_cares");
    assert!(
        count_all_not_dont_cares(&cube) == 4,
        "count all not dont_cares"
    );
}

fn most_binate_variable(F: &CubeList) -> usize {

    #[derive(Clone, PartialEq, PartialOrd, Eq, Debug)]
    struct BinateVarAttrs(isize, isize, isize); // True, Complement, Index

    #[inline]
    fn cmp_binate(left: &BinateVarAttrs, right: &BinateVarAttrs) -> Ordering {
        match (left, right) {
            (&BinateVarAttrs(t1, c1, i1), &BinateVarAttrs(t2, c2, i2)) => {
                if t2 + c2 < t1 + c1 {
                    Ordering::Less
                } else if t2 + c2 > t1 + c1 {
                    Ordering::Greater
                } else {
                    // t1+c1 == t2+c2
                    if (t1 - c1).abs() < (t1 - c1).abs() {
                        Ordering::Less
                    } else if (t1 - c1).abs() > (t1 - c1).abs() {
                        Ordering::Greater
                    } else if i1 < i2 {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
            }
        }
    }

    #[inline]
    fn cmp_unate(left: &BinateVarAttrs, right: &BinateVarAttrs) -> Ordering {
        match (left, right) {
            (&BinateVarAttrs(t1, c1, i1), &BinateVarAttrs(t2, c2, i2)) => {
                if t2 + c2 < t1 + c1 {
                    Ordering::Less
                } else if t2 + c2 > t1 + c1 {
                    Ordering::Greater
                } else {
                    // t1+c1 == t2+c2
                    if i1 < i2 {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
            }
        }
    }

    let num_cubes = F.len();
    let num_vars = if num_cubes >= 1 {
        F.front().unwrap().len()
    } else {
        0
    };
    let mut tie = vec![BinateVarAttrs(0, 0, 0); num_vars];
    for c in F.iter() {
        for i in 0..(*c).len() {
            let term = (*c)[i].clone();
            let j = i as isize;
            if term == TriLogic::True {
                tie[i] = match tie[i] {
                    BinateVarAttrs(t, c, _) => BinateVarAttrs(t + 1, c, j),
                }
            } else if term == TriLogic::False {
                tie[i] = match tie[i] {
                    BinateVarAttrs(t, c, _) => BinateVarAttrs(t, c + 1, j),
                }
            } else {
                tie[i] = match tie[i] {
                    BinateVarAttrs(t, c, _) => BinateVarAttrs(t, c, j),
                }
            }
        }
    }

    for k in 0..num_vars {
        let BinateVarAttrs(t, c, i) = tie[k];
        // println!("tie[{}] = ({},{},{})",k,t,c,i);
    }

    let there_are_binate_vars = tie.iter().any(|x| match *x {
        BinateVarAttrs(t, c, _) => t > 0 && c > 0,
    });
    let split_var_idx = if there_are_binate_vars {
        // filter tie, leave only binate cubes
        let mut tie_binate: Vec<BinateVarAttrs> = tie.iter()
            .filter(|&x| match *x {
                BinateVarAttrs(t, c, _) => t > 0 && c > 0,
            })
            .map(|x| x.clone())
            .collect();
        tie_binate.sort_by(cmp_binate);
        // println!("sorted binate: {:?}", tie_binate);
        let BinateVarAttrs(_, _, split_var_idx) = tie_binate[0];
        split_var_idx
    } else {
        tie.sort_by(cmp_unate);
        let BinateVarAttrs(_, _, split_var_idx) = tie[0];
        split_var_idx
    };
    // println!("split_var_idx = {}", split_var_idx);
    split_var_idx as usize
}

fn positiveCofactor(F: &CubeList, x: usize) -> CubeList {
    let mut cofactor_list: CubeList = LinkedList::new();
    for c in F.iter() {
        let mut cube = c.clone();
        let term_val: TriLogic = cube[x].clone();
        match term_val {
            TriLogic::True => {
                cube[x] = TriLogic::DontCare;
                cofactor_list.push_back(cube)
            }
            TriLogic::False => {} // => remove this cube
            TriLogic::DontCare => cofactor_list.push_back(cube), // leave it as is
        }
    }
    cofactor_list
}

fn negativeCofactor(F: &CubeList, x: usize) -> CubeList {
    let mut cofactor_list: CubeList = LinkedList::new();
    for c in F.iter() {
        let mut cube = c.clone();
        let term_val: TriLogic = cube[x].clone();
        match term_val {
            TriLogic::True => {} // remove this cube
            TriLogic::False => {
                cube[x] = TriLogic::DontCare;
                cofactor_list.push_back(cube)
            }
            TriLogic::DontCare => cofactor_list.push_back(cube), // leave it as is
        }
    }
    cofactor_list
}

#[test]
fn cofactors_test() {
    let clist: CubeList = LinkedList::from_iter(vec![
        vec![
            TriLogic::True,
            TriLogic::True,
            TriLogic::DontCare,
            TriLogic::True,
        ],
        vec![
            TriLogic::DontCare,
            TriLogic::True,
            TriLogic::False,
            TriLogic::DontCare,
        ],
    ]);

    let mut fa = positiveCofactor(&clist, 0);
    let fc = positiveCofactor(&clist, 2);

    assert!(fa.len() == 2);
    assert!(fc.len() == 1);
    assert!(
        fa.front().unwrap() ==
            &[
                TriLogic::DontCare,
                TriLogic::True,
                TriLogic::DontCare,
                TriLogic::True,
            ],
        "fa -- 1st cube"
    );
    assert!(
        fa.pop_back() ==
            Some(vec![
                TriLogic::DontCare,
                TriLogic::True,
                TriLogic::False,
                TriLogic::DontCare,
            ]),
        "fa -- 2nd cube"
    );
    assert!(
        fc.front().unwrap() ==
            &[
                TriLogic::True,
                TriLogic::True,
                TriLogic::DontCare,
                TriLogic::True,
            ],
        "fc -- 1st cube"
    );
}

// x offset is 1 to distinguish positive x and negative x'
fn AND(x: isize, F: &CubeList) -> CubeList {
    let ind = (x.abs() - 1) as usize;
    // check that cubelist lacks variable x
    assert!(F.iter().all(|c| c[ind] == TriLogic::DontCare));
    let mut and_list: CubeList = LinkedList::new();
    for c in F.iter() {
        let mut cube = c.clone();
        cube[ind] = if x > 0 {
            TriLogic::True
        } else {
            TriLogic::False
        };
        and_list.push_back(cube)
    }
    and_list
}

#[test]
fn AND_test() {
    let clist: CubeList = LinkedList::from_iter(vec![
        vec![
            TriLogic::DontCare,
            TriLogic::True,
            TriLogic::True,
            TriLogic::DontCare,
        ],
        vec![
            TriLogic::DontCare,
            TriLogic::DontCare,
            TriLogic::True,
            TriLogic::False,
        ],
    ]);
    let mut anda = AND(1, &clist);
    assert!(anda.len() == 2, "and_test list length check");
    assert!(
        anda.front().unwrap() ==
            &[
                TriLogic::True,
                TriLogic::True,
                TriLogic::True,
                TriLogic::DontCare,
            ],
        "and_test 1st cube"
    );
    assert!(
        anda.pop_back() ==
            Some(vec![
                TriLogic::True,
                TriLogic::DontCare,
                TriLogic::True,
                TriLogic::False,
            ]),
        "and_test 2nd cube"
    );
}

fn OR(P: &CubeList, N: &CubeList) -> CubeList {
    let mut or_list = P.clone();
    or_list.append(&mut N.clone());
    or_list
}

#[test]
fn OR_test() {
    let clist1: CubeList = LinkedList::from_iter(vec![
        vec![
            TriLogic::DontCare,
            TriLogic::True,
            TriLogic::True,
            TriLogic::DontCare,
        ],
        vec![
            TriLogic::DontCare,
            TriLogic::DontCare,
            TriLogic::True,
            TriLogic::False,
        ],
    ]);
    let clist2: CubeList = LinkedList::from_iter(vec![
        vec![
            TriLogic::True,
            TriLogic::False,
            TriLogic::True,
            TriLogic::DontCare,
        ],
        vec![
            TriLogic::DontCare,
            TriLogic::DontCare,
            TriLogic::True,
            TriLogic::False,
        ],
    ]);
    let or_list = OR(&clist1, &clist2);
    assert!(or_list.len() == 4, "or_test list length check");
}

fn direct_complement(F: &CubeList) -> CubeList {
    // check that cubelist contains exactly one cube
    assert!(F.len() == 1);
    let cube = F.front().unwrap();
    let mut result = LinkedList::new();
    let num_vars = cube.len();
    for i in 0..num_vars {
        match cube[i] {
            TriLogic::True => {
                let mut new_cube = vec![TriLogic::DontCare; num_vars];
                new_cube[i] = TriLogic::False;
                result.push_back(new_cube);
            }
            TriLogic::False => {
                let mut new_cube = vec![TriLogic::DontCare; num_vars];
                new_cube[i] = TriLogic::True;
                result.push_back(new_cube);
            }
            TriLogic::DontCare => {} // leave it as is
        }
    }
    result
}

#[test]
fn direct_complement_test() {
    let clist: CubeList = LinkedList::from_iter(vec![
        vec![
            TriLogic::DontCare,
            TriLogic::True,
            TriLogic::False,
            TriLogic::True,
        ],
    ]);
    let compl_list = direct_complement(&clist);
    assert!(compl_list.len() == 3);
    let mut cnt = 0;
    for c in compl_list.iter() {
        match (cnt, c) {
            (0, c1) => {
                assert!(
                    c1 ==
                        &[
                            TriLogic::DontCare,
                            TriLogic::False,
                            TriLogic::DontCare,
                            TriLogic::DontCare,
                        ]
                )
            }
            (1, c2) => {
                assert!(
                    c2 ==
                        &[
                            TriLogic::DontCare,
                            TriLogic::DontCare,
                            TriLogic::True,
                            TriLogic::DontCare,
                        ]
                )
            }
            (2, c3) => {
                assert!(
                    c3 ==
                        &[
                            TriLogic::DontCare,
                            TriLogic::DontCare,
                            TriLogic::DontCare,
                            TriLogic::False,
                        ]
                )
            }
            _ => panic!("impossible happened"),
        }
        cnt += 1;
    }
}

fn Complement(num_vars: usize, F: &CubeList) -> CubeList {
    let F_is_simple = || -> (bool, CubeList) {
        if F.len() == 0 {
            // empty cube list
            let mut clist: CubeList = LinkedList::new();
            clist.push_back(vec![TriLogic::DontCare; num_vars]);
            (true, clist)
        } else if cubelist_contains_all_dont_cares_cube(F) {
            (true, LinkedList::new())
        } else if F.len() == 1 {
            // cube list contains exactly one cube
            (true, direct_complement(F))
        } else {
            (false, LinkedList::new())
        }
    };

    let check_simple = F_is_simple();
    match check_simple {
        (true, x) => x,
        (false, _) => {
            // most binate variable for splitting
            let x = most_binate_variable(F);
            let mut P = Complement(num_vars, &positiveCofactor(F, x));
            let mut N = Complement(num_vars, &negativeCofactor(F, x));
            let and_ind = (x + 1) as isize;
            P = AND(and_ind, &P);
            N = AND(-and_ind, &N);
            OR(&P, &N)
        }
    }
}

fn WriteCubeList(fname: &str, F: &CubeList) -> Result<(), io::Error> {
    let mut f = File::create(fname)?;
    write!(f, "{}\n", F.front().unwrap().len())?; // num vars
    write!(f, "{}\n", F.len())?; // num cubes
    for cube in F.iter() {
        // print number of not dont_care terms
        let num_of_not_dont_cares = count_all_not_dont_cares(&*cube);
        write!(f, "{}", num_of_not_dont_cares)?;
        for i in 0..cube.len() {
            match (*cube)[i] {
                TriLogic::True => {
                    write!(f, " {}", i + 1)?;
                }
                TriLogic::False => {
                    write!(f, " {}", -(i as i32) + 1)?;
                }
                TriLogic::DontCare => {} // do nothing 
            }
        }
        write!(f, "\n")?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("plz. gimme input and output fnames");
        return;
    }

    let inp_fname = args[1].clone();
    let out_fname = args[2].clone();
    let is = File::open(inp_fname).unwrap();
    let reader = BufReader::new(is);
    let mut line_num = 0;
    let mut num_vars: usize = 0;
    let mut num_cubes: usize = 0;
    let mut cube_list: Vec<Vec<TriLogic>> = Vec::new();

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        line_num += 1;
        println!("<{}> : {}", line_num, line);
        let chunks: Vec<&str> = line.split_terminator(|c: char| c.is_whitespace()).collect();
        match line_num {
            1 => num_vars = chunks[0].parse().ok().expect("a number expected"),
            2 => {
                num_cubes = chunks[0].parse().ok().expect("a number expected");
                println!("num_vars: {} num_cubes: {}", num_vars, num_cubes);
            }
            _ => {
                if (line_num - 2) > num_cubes {
                    break;
                }
                let nterms: usize = chunks[0].parse().ok().expect("a number expected");
                let terms: Vec<isize> = Vec::from_iter((0..nterms).map(|idx| {
                    chunks[idx + 1].parse().ok().expect("a number expected")
                }));
                println!("nterms: {} terms: {:?}", nterms, terms);
                let mut cube = vec![TriLogic::DontCare; num_vars];
                for term in terms.iter() {
                    if *term > 0 {
                        cube[(*term - 1) as usize] = TriLogic::True;
                    } else {
                        cube[(-*term - 1) as usize] = TriLogic::False;
                    }
                }
                cube_list.push(cube);
            }
        }
    }
    println!("cube_list: {:?}", cube_list);
    let compl = Complement(num_vars, &LinkedList::from_iter(cube_list));
    println!("compl: {:?}", compl);

    // print it in binary form
    for cube in compl.iter() {
        print!("[ ");
        for term in cube.iter() {
            print!("{:02b} ", (*term).clone() as usize);
        }
        println!("]");
    }
    WriteCubeList(&out_fname, &compl);
}
