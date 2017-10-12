extern crate cc;

fn main() {
    cc::Build::new()
        .file("vendor/ctaes/ctaes.c")
        .compile("ctaes");
}
