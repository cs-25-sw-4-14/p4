use std::error::Error;
use std::process::Command;
use std::str;

fn main() {
    let _ = gen_for_grammar("LzyLexer", "../antlr-complete.jar", Some("-visitor"));
    let _ = gen_for_grammar("Lzy", "../antlr-complete.jar", Some("-visitor"));

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=antlr-complete.jar");
    println!("cargo:rerun-if-changed=grammar/LzyLexer.g4");
    println!("cargo:rerun-if-changed=grammar/Lzy.g4");
    println!("cargo:rerun-if-changed=grammar/ParserExpressions.g4");
    println!("cargo:rerun-if-changed=grammar/ParserStatements.g4");
}

fn gen_for_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let input = std::env::current_dir().unwrap().join("grammar");
    let file_name = grammar_file_name.to_owned() + ".g4";

    let _c = Command::new("java")
        .current_dir(input)
        .arg("-cp")
        .arg(antlr_path)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg("../src/antlr_gen")
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;

    Ok(())
}
