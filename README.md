# rust-samples
short rust code samples

![](https://github.com/alsam/rust-samples/blob/master/images/J1_int.png "J1 integrated at [0,r] when r -> ∞")

# how to run individual sample

```sh
cargo build

make af_hi
cargo run --bin af_hi
     Running `target/debug/af_hi`
Create a 5-by-3 matrix of random floats on the GPU
No Name Array
[5 3 1 1]
    0.7402     0.4464     0.7762 
    0.9210     0.6673     0.2948 
    0.0390     0.1099     0.7140 
    0.9690     0.4702     0.3585 
    0.9251     0.5132     0.6814 

cargo run --bin kernel_iterator

cargo test --bin binary_unate
     Running target/debug/binary_unate-b4436249f664116b

running 6 tests
test cofactors_test ... ok
test direct_complement_test ... ok
test AND_test ... ok
test OR_test ... ok
test test_contains_all_dont_cares_cube ... ok
test test_count_don_cares ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured

cargo run --bin binary_unate data/UnateRecursiveComplement/part1.pcn part1.out
     Running `target/debug/binary_unate data/UnateRecursiveComplement/part1.pcn part1.out`
<1> : 5
<2> : 3
num_vars: 5 num_cubes: 3
<3> : 3 2 3 4
nterms: 3 terms: [2, 3, 4]
<4> : 2 -1 5
nterms: 2 terms: [-1, 5]
<5> : 3 1 -3 -4
nterms: 3 terms: [1, -3, -4]
<6> :
cube_list: [[DontCare, True, True, True, DontCare], [False, DontCare, DontCare, DontCare, True], [True, DontCare, False, False, DontCare]]
compl: [[True, False, True, DontCare, DontCare], [True, DontCare, True, False, DontCare], [True, DontCare, False, True, DontCare], [False, True, True, False, False], [False, True, False, DontCare, False], [False, False, DontCare, DontCare, False]]
[ 01 10 01 11 11 ]
[ 01 11 01 10 11 ]
[ 01 11 10 01 11 ]
[ 10 01 01 10 10 ]
[ 10 01 10 11 10 ]
[ 10 10 11 11 10 ]
```

```sh
cargo run --bin scl -- --verbose --num_cells 10000 -b 7
     Running `target/debug/scl --verbose --num_cells 10000 -b 7`
number of time steps : 2000 number of grid cells : 10000 number of runs : 7
ijump : 1667
run number : 0
run number : 1
run number : 2
run number : 3
run number : 4
run number : 5
run number : 6
elapsed time: 28.639034207999998
```

## `scl` in *debug* vs *release* mode
```sh
./target/debug/scl --verbose --num_cells 10000 -b 7
number of time steps : 2000 number of grid cells : 10000 number of runs : 7
ijump : 1667
run number : 0
run number : 1
run number : 2
run number : 3
run number : 4
run number : 5
run number : 6
elapsed time: 28.30373625s.
./target/release/scl --verbose --num_cells 10000 -b 7
number of time steps : 2000 number of grid cells : 10000 number of runs : 7
ijump : 1667
run number : 0
run number : 1
run number : 2
run number : 3
run number : 4
run number : 5
run number : 6
elapsed time: 0.515229039s.
```

set `AF_PATH`
```sh
export AF_PATH=/usr/lib
```

## sunrise