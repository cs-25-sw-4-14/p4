use crate::{codegenerator, debugger::Debugger, lexer::Lexer, typechecker};
use anyhow::anyhow;
use lalrpop_util::lalrpop_mod;

use inkwell::context::Context;
use inkwell::targets::{FileType, InitializationConfig, Target, TargetMachine};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use tempfile::Builder; // For creating temporary directories and files

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);

// Your existing test_parser function (no changes needed for this part)
#[test]
fn test_parser() {
    let file = include_str!("integration_test.ss");
    let debugger = Debugger::new(file.to_string());

    let lexer = Lexer::new(file);
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

    let debugger = Debugger::new(file.to_string());
    let mut typechecker = typechecker::Typechecker::new(&debugger);
    typechecker
        .walk(ast.clone())
        .expect("Failed to walk the AST");
}

#[test]
fn comprehensive_integration_test() {
    // 1. Load the comprehensive source code
    let source_code = include_str!("integration_test.ss");

    // 2. Define the *expected combined output* for all debug statements
    // Make sure to match exact newlines and spacing produced by your 'debug' function.
    let expected_output = "--- Running test_literals ---12312364: 123.451097Hello, world!--- Running test_variables ---1020301RustLossy--- Running test_arithmetic ---15131251-579--- Running test_comparisons ---111111010--- Running test_if_else ---x is 10y is 5z is not greater than 5--- Running test_while_loop ---013Breaking loop--- Running test_functions ---12Hello, Lossy User--- Running test_structs ---10201002550--- Running test_arrays ---129911--- Running test_vectors ---102010150150--- Running test_references ---100100300--- Running test_type_casting ---64: 10.0015-1--- All integration tests complete ---";

    // --- Compiler Front-End (Lexing, Parsing, Type-Checking) ---
    let debugger = Debugger::new(source_code.to_string());
    let lexer = Lexer::new(source_code);
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
        .expect("Comprehensive Integration Test: Failed to parse source code.");

    let mut typechecker = typechecker::Typechecker::new(&debugger);
    typechecker
        .walk(ast.clone())
        .expect("Comprehensive Integration Test: Failed type-checking for source code.");

    // --- Compilation to Executable ---
    let temp_dir = Builder::new()
        .prefix("lossy_test_comprehensive_")
        .tempdir()
        .expect("Failed to create temporary directory.");
    let temp_dir_path = temp_dir.path();

    let object_file_path = temp_dir_path.join("output.o");
    let executable_file_path = temp_dir_path.join("output.out");

    let context = Context::create();
    let module = context.create_module("test_module_comprehensive");

    let mut codegen = codegenerator::CodeGenerator::new(&context, module);
    codegen
        .compile_program(&ast)
        .expect("Comprehensive Integration Test: Failed to generate LLVM IR.");

    Target::initialize_all(&InitializationConfig::default());
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            inkwell::OptimizationLevel::Default,
            inkwell::targets::RelocMode::Default,
            inkwell::targets::CodeModel::Default,
        )
        .unwrap();

    target_machine
        .write_to_file(
            &codegen.module,
            FileType::Object,
            Path::new(&object_file_path),
        )
        .expect("Comprehensive Integration Test: Failed to write object file.");

    let clang_output = Command::new("clang")
        .arg(&object_file_path)
        .arg("-o")
        .arg(&executable_file_path)
        .arg("-no-pie")
        .arg("-g")
        .output()
        .expect("Comprehensive Integration Test: Failed to invoke clang for linking.");

    if !clang_output.status.success() {
        eprintln!("--- Clang STDOUT ---");
        io::stderr().write_all(&clang_output.stdout).unwrap();
        eprintln!("--- Clang STDERR ---");
        io::stderr().write_all(&clang_output.stderr).unwrap();
        panic!(
            "Comprehensive Integration Test: Clang linking failed: {:?}",
            clang_output.status
        );
    }

    // --- Execution ---
    let program_output = Command::new(&executable_file_path)
        .output()
        .expect("Comprehensive Integration Test: Failed to execute compiled program.");

    // if !program_output.status.success() {
    //     eprintln!("--- Program STDOUT ---");
    //     io::stderr().write_all(&program_output.stdout).unwrap();
    //     eprintln!("--- Program STDERR ---");
    //     io::stderr().write_all(&program_output.stderr).unwrap();
    //     panic!(
    //         "Comprehensive Integration Test: Compiled program exited with an error: {:?}",
    //         program_output.status
    //     );
    // }

    // --- Assertion ---
    let actual_output = String::from_utf8_lossy(&program_output.stdout);
    assert_eq!(
        actual_output, expected_output,
        "Comprehensive Integration Test: Program output mismatch."
    );
}
