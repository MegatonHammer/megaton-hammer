extern crate cc;

fn main() {
    // TODO: Force clang here.
    if std::env::var("TARGET").unwrap() == "aarch64-roblabla-switch" {
        cc::Build::new()
            .file("src/kernel/svc.S")
            .compile("svc_asm");
    }
}
