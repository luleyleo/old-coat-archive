
include!("lib.rs");

fn main() {
    ids!(TEST, FOO, BAR);
    println!("Hello {}, {}, {}", TEST, FOO, BAR);
}
