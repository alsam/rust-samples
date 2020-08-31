extern crate gcc;

fn main() {
    gcc::Build::new()
                .file("src/ckernels.c")
                .flag("-O5")
                .compile("libckernels-add-sum-sq.a");

    println!("cargo:rustc-link-search=/opt/cuda/lib64/");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=cuda");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=cudadevrt");
}
