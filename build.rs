extern crate gcc;

use gcc::Config;

fn main() {
    Config::new()
        .cpp(true)
        .file("hellopp.cc")
        .compile("libhellopp.a");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hellopp.cc");
}
