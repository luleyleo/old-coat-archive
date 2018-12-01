
include!("lib.rs");

fn main() {
    ids!(TEST, FOO, BAR);
    println!("Hello {}, {}, {}", TEST as isize, FOO as isize, BAR as isize);
}
