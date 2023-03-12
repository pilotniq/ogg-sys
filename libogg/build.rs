extern crate pkg_config;
extern crate cc;

use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("libogg");

    println!("cargo:include={}", root.join("include").display());

    match pkg_config::find_library("ogg") {
        Ok(_) => return,
        Err(..) => {}
    };

    cc::Build::new()
        .compiler("xtensa-esp32-elf-cc")
                .file("libogg/src/bitwise.c")
                .file("libogg/src/framing.c")
                .define("_USRDLL", None)
                .define("LIBOGG_EXPORTS", None)
                .include(&root.join("include"))
                .compile("libogg.a");
}
