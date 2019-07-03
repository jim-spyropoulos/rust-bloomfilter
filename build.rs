extern crate cc;

fn main() {
    
      cc::Build::new()
          .file("src/xxhash.c")
          .compile("libdouble.a");
}
