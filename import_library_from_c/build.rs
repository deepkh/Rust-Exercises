//use std::process::Command;
//use std::env;
//use std::path::Path;

fn main() {
    println!("cargo:rustc-link-search=native={}", "./");
    println!("cargo:rustc-link-lib=mylib");
}

