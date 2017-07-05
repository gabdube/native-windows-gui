extern crate build;
fn main() {
    if cfg!(feature = "canvas") {
        build::link("d2d1", true);
    }
}