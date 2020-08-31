#![allow(incomplete_features)]
#![feature(const_generics)]
struct MyVec<T: Sized, const LENGTH: usize> {
    inner_data: [T; LENGTH],
}

impl<T, const L: usize> MyVec<T, L> {
    pub fn new(value: T) -> Self {
        MyVec {
            inner_data: [value; L],
        }
    }
}

fn main() {
    let _my_vec = MyVec::<f64, 10>::new(4.2);
}
