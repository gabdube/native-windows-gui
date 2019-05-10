extern crate build;
fn main() {
    build::link("shell32", true);
    build::link("d2d1", true);
}