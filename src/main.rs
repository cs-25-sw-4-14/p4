pub mod antlr_gen;
pub mod typechecker;

use antlr_gen::lzylexer::*;
use antlr_gen::lzyparser::*;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;
use typechecker::function_table::FunctionTableVisitor;

const TEST_FILE: &str = include_str!("../tests/test.lzy");

fn main() {
    let input = InputStream::new(TEST_FILE.into());
    let lexer = LzyLexer::new(input);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LzyParser::new(token_source);
    let result = parser.program().unwrap();
    let mut visitor = FunctionTableVisitor::new();
    visitor.visit(&*result);
    println!("Function Table:\n{}", visitor);
    // let tree = result.to_string_tree(&*parser)
    // println!("Parse Tree: {}", tree);
}
