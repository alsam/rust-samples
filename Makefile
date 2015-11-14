#
#  Makefile for running rust samples
#

build_all:
	cargo build

build_all_release:
	cargo build release

array_vec:
	cargo run --bin array_vec_demo

dllist:
	cargo run --bin dllist_demo

kernel_iterator:
	cargo run --bin kernel_iterator

binary_unate_test:
	cargo test --bin binary_unate

binary_unate:
	cargo run --bin binary_unate data/UnateRecursiveComplement/part1.pcn part1.out

scl:
	cargo run --bin scl -- --verbose --num_cells 10000 -b 7

scl_dbg:
	./target/debug/scl --verbose --num_cells 10000 -b 7

scl_rel:
	./target/release/scl --verbose --num_cells 10000 -b 7

