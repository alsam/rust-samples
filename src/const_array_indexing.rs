fn main() {
    const ARR: [usize; 5] = [1, 2, 3, 4, 5];
    const ARR2: [usize; ARR[1]] = [42, 99];
}

// Compiling playground v0.0.1 (/playground)
// warning: constant is never used: `ARR`
//  --> src/main.rs:2:5
//   |
// 2 |     const ARR: [usize; 5] = [1, 2, 3, 4, 5];
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   |
//   = note: `#[warn(dead_code)]` on by default
// 
// warning: constant is never used: `ARR2`
//  --> src/main.rs:3:5
//   |
// 3 |     const ARR2: [usize; ARR[1]] = [42, 99];
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// 
// warning: 2 warnings emitted
// 
//     Finished dev [unoptimized + debuginfo] target(s) in 0.73s
//      Running `target/debug/playground`
