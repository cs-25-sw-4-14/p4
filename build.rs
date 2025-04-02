use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use build_print::{info, warn};

fn main() {
    let crate_root: PathBuf = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let grammar_dir = crate_root.join("grammar");

    let lzy = grammar_dir.join("Lzy.g4");
    let lzy_lexer = grammar_dir.join("LzyLexer.g4");

    generate_antlr(&lzy, &crate_root);
    generate_antlr(&lzy_lexer, &crate_root);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=grammar/*");
    println!("cargo:rerun-if-changed=antlr-complete.jar");
}

fn generate_antlr(grammar_path: &Path, root_dir: &Path) {
    let antlr_jar = root_dir.join("antlr-complete.jar");
    let output_dir = root_dir.join("src").join("antlr_gen");
    let output = Command::new("java")
        .current_dir(output_dir.clone())
        .arg("-cp")
        .arg(antlr_jar)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg(output_dir)
        .arg(grammar_path)
        .output()
        .expect("antlr tool failed to start");

    if output.status.success() {
        info!("ANTLR generated files successfully for {}", grammar_path.display());
    } else {
        let error_message = str::from_utf8(&output.stderr).unwrap();
        warn!("ANTLR: {}", error_message);
    }
}
