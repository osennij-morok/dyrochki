use std::{fs, path::Path};

use sha1::Digest;

fn main() {
    let bulma_css_hash: String = hash_file("./static/css/bulma.min.css");
    println!("cargo:rustc-env=BULMA_HASH={}", bulma_css_hash);
    let favicon_hash: String = hash_file("./static/favicon.png");
    println!("cargo:rustc-env=FAVICON_HASH={}", favicon_hash);
    println!("cargo:rustc-env=RUST_LOG=info");
}

fn hash_file<P: AsRef<Path>>(path: P) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(fs::read(path).unwrap());
    let hash = hasher.finalize();
    hex::encode(hash)
}