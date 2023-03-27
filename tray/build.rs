extern crate embed_resource;

use copy_to_output::copy_to_output;
use embed_manifest::embed_manifest_file;

use std::env;

fn main() {
    // println!("cargo:rerun-if-changed=app-name-manifest.rc");

    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // embed_manifest_file("./game-time-manager-tray.manifest")
        //     .expect("unable to embed manifest file");
        embed_resource::compile("game-time-manager-tray-manifest.rc", embed_resource::NONE);
    }

    let proj_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rerun-if-changed=./src");
    let incl_files = ["src/icons", "src/config.toml"];

    for file in incl_files.iter() {
        copy_to_output(
            &format!("{}/{}", proj_root, file),
            &env::var("PROFILE").unwrap(),
        )
        .expect(&format!("Could not copy {}", file));
    }

    // println!("cargo:rerun-if-changed=app-name-manifest.rc");
    // embed_resource::compile("game-time-manager-tray-manifest.rc", embed_resource::NONE);
}
