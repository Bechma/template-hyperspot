use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let cargo_toml_path = PathBuf::from(&manifest_dir).join("Cargo.toml");

    println!("cargo:rerun-if-changed={}", cargo_toml_path.display());

    let manifest =
        cargo_toml::Manifest::from_path(&cargo_toml_path).expect("Failed to parse Cargo.toml");

    let mut cf_modules = Vec::new();

    for (dep_name, _dep_detail) in manifest.dependencies {
        if dep_name.starts_with("cf_") {
            cf_modules.push(dep_name);
            continue;
        }
    }

    cf_modules.sort();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("registered_modules.rs");

    let mut content = String::new();
    const UNUSED_IMPORT: &str = "#[allow(unused_imports)]\n";

    if !cf_modules.is_empty() {
        content.push_str("// CyberFabric modules (cf-*)\n");
        for module in &cf_modules {
            content.push_str(&format!("{UNUSED_IMPORT}use {} as _;\n", module));
        }
        content.push('\n');
    }

    fs::write(&dest_path, content).expect("Failed to write registered_modules.rs");

    println!(
        "cargo:info=Generated registrations - cf modules: {cf_modules:?}",
    );
}
