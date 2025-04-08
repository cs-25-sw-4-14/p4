use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser::lossy);
pub mod ast;
pub mod lexer;
pub mod token;

fn main() {}

#[test]
fn calculator4() {
    let expr = lossy::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
