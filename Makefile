#
#  Makefile for running rust samples
#

## CC = gcc
## #CFLAGS = -O3 -fomit-frame-pointer -march=native -mfpmath=sse -msse2 -ftree-vectorize -ftree-vectorizer-verbose=7 -fopt-info-vec-missed -Wall
## CFLAGS = -O3 -fomit-frame-pointer -march=native -mfpmath=sse -msse2 -ftree-vectorize -fopt-info-vec -Wall

CC = clang
CFLAGS = -O3 -mllvm -force-vector-width=2
#CFLAGS = -O5
#CFLAGS = -O3 -mllvm -fslp-vectorize-aggressive


build: ckernels
	cargo build

build-nightly: ckernels
	cargo build --features use-nightly

release: ckernels
	cargo build --release

clippy:
	cargo clippy

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

adt_enum:
	cargo run --bin adt_enum -- mask_after_biasing_number17_bias_-2.000000_grid.bin -d data/grid/
	asy -f pdf mask_after_biasing_number17_bias_-2.000000_grid.bin.asy

af_hi:
	cargo run --bin af_hi

af_fft:
	cargo run --bin af_fft

jinc:
	cargo run --bin jinc

tiny:
	cargo run --bin tiny

serde:
	cargo run --bin serde_yaml_json

ndarray:
	cargo run --bin ndarray_demo

simd:
	time cargo run --release --bin nbody 100000000
	time cargo run --release --bin nbody_simd 100000000

ckernels:
	mkdir -p target/debug/ target/release/
	$(CC) $(CFLAGS) -g -shared src/ckernels.c -o target/debug/libckernels.so
	$(CC) $(CFLAGS) -shared src/ckernels.c -o target/release/libckernels.so
	cp target/release/libckernels.so ~/lib/

add_sum_sq: ckernels
	cargo run --verbose --release --bin add_sum_sq
	#cargo run --verbose --release --bin add_sum_sq -- -v -i data/add_sum_sq/sum_ef_20085.bin

add_sum_check: ckernels
	cargo run --verbose --release --bin add_sum_sq -- -v -n 1 -k 5 -i `pwd`/data/add_sum_sq/sum_ef_20085.bin > _ai_out5_
	cargo run --verbose --release --bin add_sum_sq -- -v -n 1 -k 7 -i `pwd`/data/add_sum_sq/sum_ef_20085.bin > _ai_out7_


add_sum_sq_profile: ckernels
	cargo profiler cachegrind --bin ./target/debug/add_sum_sq -- -k 1 -i `pwd`/data/add_sum_sq/sum_ef_20085.bin

add_sum_sq_bench: ckernels
	cargo bench --verbose --bin add_sum_sq

add_sum_sq_operf: ckernels
	operf ./target/debug/add_sum_sq -k 4 -n 100
	opannotate --source --assembly > operf.listing

%.pdf : %.md
	pandoc --include-in-header=fontoptions.tex -s -t beamer  -V theme:Warsaw --highlight-style pygments  --latex-engine=xelatex  $< -o $@

clean:
	cargo clean
	cargo clean --release
