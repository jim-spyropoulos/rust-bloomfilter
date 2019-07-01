extern crate cc;

fn main() {
    // cc::Build::new()
    //     .file("src/double.c")
    //     .compile("libdouble.a");
    
    //println!("cargo:rustc-link-search=/Users/jJimo/Downloads/xxHash-dev/");

      cc::Build::new()
          .file("src/xxhash.c")
          .compile("libdouble.a");
}