use anyhow::anyhow;
use clap::Parser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);
pub mod ast;
pub mod debugger;
pub mod typechecker;

/// The Lossy compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to compile
    input: String,
}

fn main() {
    let huttel = include_str!("huttel.txt");
    let args = Args::parse();
    let file = std::fs::read_to_string(&args.input).unwrap_or_else(|_| {
        panic!("Failed to read file: {}", &args.input);
    });
    let debugger = debugger::Debugger::new(file.to_string());

    let ast = lossy::ProgramParser::new()
        .parse(&file)
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

    let mut typechecker = typechecker::Typechecker::new();
    typechecker.walk(ast.clone(), debugger).expect(&format!(
        "{}\nSemantic check failed!!! You dork!!!: ",
        &huttel
    ));
}
