use copy_to_output::copy_to_output;
use embed_manifest::embed_manifest_file;
use std::env;

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest_file("./game-time-manager.exe.manifest")
            .expect("unable to embed manifest file");
    }

    let proj_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let incl_files = [
        "src/config.toml",
        "src/install.ps1",
        "src/uninstall.ps1",
        "src/fonts",
    ];

    for file in incl_files.iter() {
        copy_to_output(
            &format!("{}/{}", proj_root, file),
            &env::var("PROFILE").unwrap(),
        )
        .expect(&format!("Could not copy {}", file));
    }

    println!("cargo:rerun-if-changed=./src");
}
