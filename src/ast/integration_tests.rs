use crate::{lexer::Lexer, typechecker};
use anyhow::anyhow;
use lalrpop_util::lalrpop_mod;

use crate::debugger;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);

#[test]
fn test_parser() {
    let file = include_str!("integration_test.ss");
    let debugger = debugger::Debugger::new(file.to_string());

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

    let mut typechecker = typechecker::Typechecker::new();
    typechecker
        .walk(ast.clone(), debugger)
        .expect("Failed to walk the AST");
}
