fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_language(0x0804);
        res.compile().unwrap();
    }

    let clib = cmake::Config::new("src/c").build();
    println!("cargo:rustc-link-search=native={}", clib.display());
    println!("cargo:rustc-link-lib=static=yztsec");
}
