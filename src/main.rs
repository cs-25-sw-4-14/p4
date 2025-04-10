use lalrpop_util::lalrpop_mod;
use std::process;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);

pub mod ast;
pub mod typechecker;

fn main() {
    let file = match std::fs::read_to_string("test.lzy") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading test.lzy: {}", err);
            process::exit(1);
        }
    };
    
    let expr = match lossy::ProgramParser::new().parse(file.as_str()) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Failed to parse: {}", err);
            process::exit(1);
        }
    };

    let mut typechecker = typechecker::Typechecker::new();

    match typechecker.walk(expr.clone()) {
        Ok(_) => {
            println!("Type checking successful!");
        },
        Err(err) => {
            eprintln!("Type checking error: {}", err);
            process::exit(1);
        }
    }

    println!("Program is valid!");
    // println!("{:#?}", expr);
    // println!("Function Table: {:#?}", typechecker.ftable.functions);
    // println!("Struct Table: {:#?}", typechecker.stable.structs);
}
