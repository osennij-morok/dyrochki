use std::{fs, path::Path};

use sha1::Digest;

fn main() {
    let bulma_css_hash: String = hash_file("./static/css/bulma.min.css");
    println!("cargo:rustc-env=BULMA_HASH={}", &bulma_css_hash);
}

fn hash_file<P: AsRef<Path>>(path: P) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(fs::read_to_string(path).unwrap());
    let hash = hasher.finalize();
    hex::encode(hash)
}