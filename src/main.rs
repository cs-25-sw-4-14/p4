use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub lossy
);
pub mod ast;
pub mod typechecker;

fn main() {
let file = std::fs::read_to_string("test.lzy").unwrap();
let expr = lossy::ProgramParser::new()
        .parse(file.as_str())
        .expect("Failed to parse: ");

println!("{:#?}", expr);
}

