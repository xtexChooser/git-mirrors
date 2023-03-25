use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/netinet.h");
    let bindings = bindgen::Builder::default()
        .header("src/netinet.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type("in6_addr.*")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("netinet.rs")).unwrap();
}
