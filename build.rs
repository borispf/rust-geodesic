extern crate gcc;

fn main() {
    gcc::compile_library("libgeodesic.a", &["src/geodesic.c", "src/geodesic.c"]);
}
