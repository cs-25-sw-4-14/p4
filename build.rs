use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::{fs, str};

fn main() {
    let out_dir = Path::new("src/gen");
    fs::create_dir_all(&out_dir).unwrap();

    let _ = gen_for_grammar("LzyLexer", "../antlr-complete.jar", Some("-visitor"));
    let _ = gen_for_grammar("Lzy", "../antlr-complete.jar", Some("-visitor"));

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=antlr-complete.jar");
    println!("cargo:rerun-if-changed=grammar/LzyLexer.g4");
    println!("cargo:rerun-if-changed=grammar/Lzy.g4");
    println!("cargo:rerun-if-changed=grammar/ParserExpressions.g4");
    println!("cargo:rerun-if-changed=grammar/ParserStatements.g4");

    for file in &["lzylexer.rs", "lzyparser.rs", "lzylistener.rs", "lzyvisitor.rs"] {
        let path = out_dir.join(file);
        let content = fs::read_to_string(&path).unwrap();
        let patched = fix_inner_attrs(&content);
        fs::write(path, patched).unwrap();
    }
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
        .arg("../src/gen")
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;


    eprintln!("{}", str::from_utf8(&_c.stdout).unwrap());


    Ok(())
}

/// Replace all inner attributes (#![...]) with outer ones (#[...])
fn fix_inner_attrs(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("#![") {
                line.replacen("#![", "#[", 1)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
