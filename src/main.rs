mod antlr_gen;
mod typechecking;

use antlr_gen::lzylexer::*;
use antlr_gen::lzyparser::*;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::trees;
use antlr_rust::InputStream;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::ParseTree;

const TEST_FILE: &str = include_str!("../tests/test.lzy");

fn main() {
    let input = InputStream::new(TEST_FILE.into());
    let mut lexer = LzyLexer::new(input);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LzyParser::new(token_source);
    let tree = parser.program().expect("parser error");

    println!("Parse tree: {}", tree.to_string_tree(&parser));
    
}

