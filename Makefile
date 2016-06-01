#
#  Makefile for running rust samples
#

CC = gcc
CFLAGS = -O5 -Wall

build: ckernels
	cargo build

release: ckernels
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

union:
	RUST_BACKTRACE=1 cargo run --bin union

af_hi:
	cargo run --bin af_hi

af_fft:
	cargo run --bin af_fft

jinc:
	cargo run --bin jinc

tiny:
	cargo run --bin tiny

ndarray:
	cargo run --bin ndarray_demo

simd:
	time cargo run --release --bin nbody 100000000
	time cargo run --release --bin nbody_simd 100000000

ckernels:
	$(CC) $(CFLAGS) -g -shared src/ckernels.c -o target/debug/libckernels.so
	$(CC) $(CFLAGS) -shared src/ckernels.c -o target/release/libckernels.so

add_sum_sq: ckernels
	cargo run --release --bin add_sum_sq -- -v -i data/add_sum_sq/sum_ef_20085.bin

add_sum_sq_profile: ckernels
	cargo profiler cachegrind --bin ./target/debug/add_sum_sq -- -k 1 -i `pwd`/data/add_sum_sq/sum_ef_20085.bin


%.pdf : %.md
	pandoc --include-in-header=fontoptions.tex -s -t beamer  -V theme:Warsaw --highlight-style pygments  --latex-engine=xelatex  $< -o $@

clean:
	cargo clean
	cargo clean --release
