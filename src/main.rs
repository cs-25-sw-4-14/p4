pub mod antlr_gen;
pub mod typechecker;


use antlr_gen::lzylexer::LzyLexer;
use antlr_gen::lzyparser::LzyParser;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

const TEST_FILE: &str = include_str!("../tests/test.lzy");

fn main() {
    let input = InputStream::new(TEST_FILE.into());
    let lexer = LzyLexer::new(input);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LzyParser::new(token_source);

    let result = parser.program().unwrap();

    typechecker::TypeChecker::check(result);

    // let tree = result.to_string_tree(&*parser);
    // println!("Parse Tree: {}", tree);
}
