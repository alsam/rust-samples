# rust-samples
short rust code samples

# how to run individual sample

```sh
cargo build
cargo run --bin kernel_iterator

cargo test --bin binary_unate
     Running target/debug/binary_unate-b4436249f664116b

running 3 tests
test test_count_don_cares ... ok
test test_contains_all_dont_cares_cube ... ok
test cofactors_test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured

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
tie[0] = (1,1,0)
tie[1] = (1,0,1)
tie[2] = (1,1,2)
tie[3] = (1,1,3)
tie[4] = (1,0,4)
sorted binate: [BinateVarAttrs(1, 1, 0), BinateVarAttrs(1, 1, 2), BinateVarAttrs(1, 1, 3)]
split_var_idx = 0
```

