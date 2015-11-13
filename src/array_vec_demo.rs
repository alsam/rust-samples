// partially borrowed from
// https://doc.rust-lang.org/std/slice/
// http://rustbyexample.com/array.html
extern crate num;
use num::{Num, Zero, One, Signed, NumCast};

use std::iter::FromIterator;
use std::cmp::Ordering;
use std::slice::Windows;
use std::ops::{Add, Sub, Mul};

//impl Sub<Vec<f64>> for Vec<f64> {
//    type Output = Self;
//    fn sub(self, rhs: Self) -> Self {
//        Vec::from_iter (self.iter().zip(rhs.iter()) .map(|(&x,&y)| x - y ) )
//    }
//}


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
    let v = Vec::from_iter ((0..10) .map (|idx| vec![idx; idx * 2]));
    println!("v.len(): {} v[1].len(): {}", v.len(), v[1].len());
    println!("v: {:?}", v);
    // prints
    // v: [[], [1, 1], [2, 2, 2, 2], [3, 3, 3, 3, 3, 3], [4, 4, 4, 4, 4, 4, 4, 4],
    // [5, 5, 5, 5, 5, 5, 5, 5, 5, 5], [6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6],
    // [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
    // [8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
    // [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]]

    let v22 = v[2][2];
    println!("v22: {}", v22);

    // fill a non-equidistant array
    let dx = std::f64::consts::FRAC_PI_4; // 0.785398
    let mut x = Vec::from_iter ((1..7) .map(|idx| ((idx as f64).sin() * dx).cos().abs() ) );
    println!("x: {:?}", x);

    // x.sort(); // error: the trait `core::cmp::Ord` is not implemented for the type `f64` [E0277]
    x.sort_by(|&x,&y|
        if      x < y { Ordering::Less }
        else if x > y { Ordering::Greater }
        else { Ordering::Equal } );

    println!("ordered x: {:?}", x);

    let dxv = Vec::from_iter ((0..x.len()-1) .map(|idx| (x[idx+1]-x[idx])));

    println!("dxv: {:?}", dxv);

    let dxv1 = Vec::from_iter (x.windows(2) .map(|w| (w[1]-w[0])));

    println!("dxv1: {:?}", dxv);

    //let mut dx = std::f64::MAX;
    // minimum of Vectors
    let dx = dxv1.iter().fold(std::f64::MAX, |dx, x| dx.min(*x));

    println!("dx = {}", dx);

    // put it together
    let dx1 = (x.windows(2) .map(|w| (w[1]-w[0]))) .fold(std::f64::MAX, |x1, x2| x1.min(x2));
    println!("dx1 = {}", dx1);

    fn sub<T: Num + Copy>(lhs: &[T], rhs: &[T]) -> Vec<T> {
        Vec::from_iter (lhs.iter().zip(rhs.iter()) .map(|(&x,&y)| x.sub(y) ) )
    }

    let delta_x = sub( &[0.07295467223444416, 0.07556433468867207, 0.07894463883160331,
                         0.08284895327495356, 0.09760168013869176, 0.09938640428738624], &x);

    println!("delta_x: {:?}",delta_x);

    let inf_norm =  delta_x .iter() .fold(delta_x[0].abs(), |x1, x2| (x1).max((*x2).abs()));
    println!("inf_norm : {}", inf_norm);

    fn infinity_norm<T: Signed+PartialOrd>(v: &[T]) -> T {
        // the trait `Signed` does have `abs` but doesn't have `max`
        //let inf_norm = v.iter() .fold(v[0].abs(), |x1, x2| (x1).max((*x2).abs()));
        let inf_norm = v.iter()
            .map(|x| (*x).abs())
            .fold(v[0].abs(), |x1, x2| if x1 > x2 { x1 } else { x2 });
        inf_norm
    }

    println!("infinity_norm : {}", infinity_norm(&delta_x));

}
