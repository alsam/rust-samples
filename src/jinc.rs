extern crate num;

extern crate special_fun;

use special_fun::FloatSpecial;

fn main() {
    let v = 0.5f64.besselj(1.0);
    println!("besselj(1.0, 0.5) : {}", v);
}
