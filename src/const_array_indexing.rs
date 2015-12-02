#![feature(const_indexing)]

// used to have
// const_arr.rs:5:24: 5:30 error: array length constant evaluation error: unsupported constant expr [E0250]
// const_arr.rs:5    const ARR2: [usize; ARR[1]] = [42, 99];
//                                       ^~~~~~
// const_arr.rs:5:24: 5:30 help: run `rustc --explain E0250` to see a detailed explanation

fn main() {
    const ARR: [usize; 5] = [1, 2, 3, 4, 5];
    const ARR2: [usize; ARR[1]] = [42, 99];
}
