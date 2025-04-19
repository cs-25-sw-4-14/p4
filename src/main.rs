use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);
pub mod ast;
pub mod typechecker;
pub mod debug;

fn main() {
    let huttel = include_str!("huttel.txt");
    let file = include_str!("test.ss");
    let expr = lossy::ProgramParser::new()
        .parse(file)
        .expect(&format!("{}\nFailed to parse: ", &huttel));

    let mut typechecker = typechecker::Typechecker::new();

    typechecker.walk(expr.clone());

    for def in &expr.definitions {
        if let ast::TopLevelDefinition::Function(func_def) = def {
            for stmt in &func_def.body {
                match typechecker::Typechecker::check_breaks_continues(&stmt, false, file) {
                    Ok(()) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
