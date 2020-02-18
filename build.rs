extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/geodesic.c")
        .compile("libgeodesic.a");
}
