use std::{fs, path::Path};

pub fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    println!("cargo:rustc-env=ASSET_OUT_DIR={}", out_dir.display());
    println!("cargo:rerun-if-changed=../../bun.lock");
    println!("cargo:rerun-if-changed=../../package.json");

    println!("cargo:rerun-if-changed=../../assets/style.scss");
    let css = grass::from_path("../../assets/style.scss", &grass::Options::default())
        .expect("Failed to compile SCSS");
    fs::write(out_dir.join("style.css"), css).expect("Failed to write style.css");
}
