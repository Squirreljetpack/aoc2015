// use std::env;
use std::path::Path;

fn main() {
    let dir = "/opt/homebrew/lib";
    if Path::new(dir).exists() {
        println!("cargo:rustc-link-search=native={}", dir);
    }
}