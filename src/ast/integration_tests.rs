use crate::typechecker;
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
    let huttel = include_str!("../huttel.txt");
    let file = include_str!("test.ss");
    let debugger = debugger::Debugger::new(file.to_string());

    let ast = lossy::ProgramParser::new()
        .parse(file)
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
