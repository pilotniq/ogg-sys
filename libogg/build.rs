extern crate "pkg-config" as pkg_config;
extern crate gcc;

fn main() {
    let root = Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("libogg");

    println!("cargo:include={}", root.join("include").display());

    match pkg_config::find_library("ogg") {
        Ok(_) => return,
        Err(..) => {}
    };

    gcc::Config::new()
                .file("libogg/src/bitwise.c")
                .file("libogg/src/framing.c")
                .define("_USRDLL", None)
                .define("LIBOGG_EXPORTS", None)
                .include(root.join("include"))
                .compile("libogg.a");
}
