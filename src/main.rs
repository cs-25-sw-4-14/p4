use crate::lexer::Lexer;
use anyhow::anyhow;
use clap::Parser;
use inkwell::OptimizationLevel;
use inkwell::targets::{FileType, InitializationConfig, Target, TargetMachine};
use lalrpop_util::lalrpop_mod;
use std::path::Path;
use std::process::Command;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);
pub mod ast;
pub mod codegenerator;
pub mod debugger;
pub mod lexer;
pub mod tokens;
pub mod typechecker;

use anyhow::Result;

/// The Lossy compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to compile
    input: String,
}

fn main() -> Result<()> {
    let huttel = include_str!("unknown.txt");
    let args = Args::parse();
    let file = std::fs::read_to_string(&args.input).unwrap_or_else(|_| {
        panic!("Failed to read file: {}", &args.input);
    });
    let debugger = debugger::Debugger::new(file.to_string());

    let lexer = Lexer::new(&file);
    let parser = lossy::ProgramParser::new();
    let ast = parser
        .parse(lexer)
        .map_err(|err| match err {
            lalrpop_util::ParseError::InvalidToken { location } => {
                anyhow!("Invalid token at {}", debugger.display(location))
            }
            lalrpop_util::ParseError::UnrecognizedEof {
                location,
                expected: _,
            } => {
                anyhow!("Unrecognized EOF at {}", debugger.display(location))
            }
            lalrpop_util::ParseError::UnrecognizedToken { token, expected: _ } => {
                anyhow!(
                    "Unrecognized token at {}: '{}'",
                    debugger.display(token.0),
                    token.1
                )
            }
            lalrpop_util::ParseError::ExtraToken { token } => {
                anyhow!(
                    "Extra token at {}: '{}'",
                    debugger.display(token.0),
                    token.1
                )
            }
            lalrpop_util::ParseError::User { error } => {
                anyhow!("User error {:?}", error)
            }
        })
        .expect("Failed to parse:");

    //Print AST
    println!("\nAST:\n{:#?}", ast);

    let mut typechecker = typechecker::Typechecker::new(&debugger);
    typechecker
        .walk(ast.clone())
        .map_err(|e| anyhow!("\n{}{}", &huttel, e))?;

    let ctx = inkwell::context::Context::create();
    let module = ctx.create_module("program");
    let mut codegen = codegenerator::CodeGenerator::new(&ctx, module);
    codegen
        .compile_program(&ast)
        .expect("Failed to compile program");

    let output = codegen.module.print_to_string().to_string();
    println!("\nIR code:\n{}", output);

    Target::initialize_all(&InitializationConfig::default());

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            inkwell::targets::RelocMode::Default,
            inkwell::targets::CodeModel::Default,
        )
        .unwrap();

    let filename = args.input.split('.').next().unwrap_or(&args.input);
    let object_file = format!("{}.o", filename);
    let output_file = format!("{}.out", filename);

    target_machine
        .write_to_file(&codegen.module, FileType::Object, Path::new(&object_file))
        .unwrap();

    Command::new("clang")
        .arg(&object_file)
        .arg("-o")
        .arg(&output_file)
        .arg("-no-pie")
        .arg("-g")
        .status()
        .expect("Failed to invoke clang");

    Command::new("rm")
        .arg(&object_file)
        .status()
        .expect("Failed to delete object file");

    println!(
        "Program succesfully compiled and saved as '{}'",
        &output_file
    );
    Ok(())
}
