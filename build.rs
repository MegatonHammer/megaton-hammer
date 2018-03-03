extern crate cc;

fn main() {
    // TODO: Force clang here.
    cc::Build::new()
        .file("src/kernel/svc.S")
        .compile("svc_asm");
}
