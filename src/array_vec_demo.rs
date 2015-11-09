// partially borrowed from
// https://doc.rust-lang.org/std/slice/
// http://rustbyexample.com/array.html
use std::iter::FromIterator;

fn main() {
    // slicing a Vec
    let vec = vec![1, 2, 3];
    let int_slice = &vec[..];
    // coercing an array to a slice
    let str_slice: &[&str] = &["one", "two", "three"];
    
    let x = &mut [1, 2, 3];
    println!("x: {:?}", x);
    x[1] = 7;
    assert_eq!(x, &[1, 7, 3]);

    println!("x: {:?}", x);

    let mut arr1 = [1, 2];
    println!("arr1: {:?}", arr1);
    arr1[1] = 1;
    println!("arr1: {:?}", arr1);

    let mut vec1 = [1, 2];
    println!("vec1: {:?}", vec1);
    vec1[1] = 1;
    println!("vec1: {:?}", vec1);

    //let mut vec2 = vec![|&i| vec![0; i]; 7];
    let mut vec2 = vec![vec![0; 7]; 7];

    // how to create a ragged array
    let mut v = Vec::from_iter ((0..10) .map (|idx| vec![idx; idx * 2]));
    println!("v.len(): {} v[1].len(): {}", v.len(), v[1].len());
    println!("v: {:?}", v);
    // prints
    // v: [[], [1, 1], [2, 2, 2, 2], [3, 3, 3, 3, 3, 3], [4, 4, 4, 4, 4, 4, 4, 4],
    // [5, 5, 5, 5, 5, 5, 5, 5, 5, 5], [6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6],
    // [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
    // [8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
    // [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]]

    let v22 = v[2][2];
    println!("v22: {}", v22)
}
