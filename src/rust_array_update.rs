// just proving that that array update works

// https://doc.rust-lang.org/std/slice/
// http://rustbyexample.com/array.html
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
}
