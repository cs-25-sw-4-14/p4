use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);
pub mod ast;
pub mod codegenerator;
pub mod typechecker;

fn main() {
    let huttel = include_str!("huttel.txt");
    let file = include_str!("test.ss");
    let expr = lossy::ProgramParser::new()
        .parse(file)
        .expect(&format!("{}\nFailed to parse: ", &huttel));

    let mut typechecker = typechecker::Typechecker::new();

    typechecker.walk(expr.clone());

    println!("{:#?}", expr);
    println!("Function Table: {:#?}", typechecker.ftable.functions);
    println!("Struct Table: {:#?}", typechecker.stable.structs);
}
