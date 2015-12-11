#
#  Makefile for running rust samples
#

build:
	cargo build

release:
	cargo build --release

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

scl_test:
	cargo test --bin scl

scl_tr_test:
	cargo test --bin scl_tr

scl:
	cargo run --bin scl -- --verbose --num_cells 10000 -b 7

scl_dbg:
	./target/debug/scl --verbose --num_cells 10000 -b 7

scl_rel:
	./target/release/scl --verbose --num_cells 10000 -b 7

scl_tr_rel:
	./target/release/scl_tr --verbose --num_cells 10000 -b 7

convolve:
	cargo run --bin convolve

fft_kernels:
	cargo run --bin fft_kernels -- --verbose  --iter 10000 --tol 1e-12

generics:
	RUST_BACKTRACE=1 cargo run --bin generics

af_hi:
	cargo run --bin af_hi

af_fft:
	cargo run --bin af_fft

%.pdf : %.md
	pandoc --include-in-header=fontoptions.tex -s -t beamer  -V theme:Warsaw --highlight-style pygments  --latex-engine=xelatex  $< -o $@

clean:
	cargo clean
	cargo clean --release
