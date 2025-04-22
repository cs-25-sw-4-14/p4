use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);
pub mod ast;
pub mod debugger;
pub mod typechecker;

fn main() {
    let huttel = include_str!("huttel.txt");
    let file = include_str!("test.ss");
    let debugger = debugger::Debugger::new(file.to_string());

    let expr = lossy::ProgramParser::new()
        .parse(file)
        .expect(&format!("{}\nFailed to parse: ", &huttel));

    let mut typechecker = typechecker::Typechecker::new();

    typechecker.walk(expr.clone(), debugger).expect(&format!(
        "{}\nSemantic check failed!!! You dork!!!: ",
        &huttel
    ));
}
