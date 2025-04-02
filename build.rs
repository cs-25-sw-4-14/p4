use std::env;
use std::process::Command;
use std::str;
use build_print::{error, info};

fn main() {

    let grammar_dir = env::current_dir().unwrap().join("grammar");

    let lzy = grammar_dir.join("lzy.g4");
    let lzy_lexer = grammar_dir.join("lzyLexer.g4");

    generate_antlr(lzy.to_str().unwrap());
    generate_antlr(lzy_lexer.to_str().unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=grammar/*");
    println!("cargo:rerun-if-changed=antlr-complete.jar");
}

fn generate_antlr(grammar_path: &str) {
    let cwd = env::current_dir().unwrap().join("gen");
    let antlr_jar = env::current_dir().unwrap().join("antlr-complete.jar");
    let output_dir = env::current_dir().unwrap().join("gen");
    let output = Command::new("java")
        .current_dir(cwd.clone())
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
        info!("ANTLR generated files successfully for {}", grammar_path);
    } else {
        let error_message = str::from_utf8(&output.stderr).unwrap();
        error!("ANTLR failed to generate files with error: {}", error_message);
    }
}
