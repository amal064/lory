use insta::assert_debug_snapshot;
use lory::{lexer, parser};

macro_rules! test_expression_from_file {
    ($file:expr) => {{
        let tokens = lexer::tokenize(include_str!($file));
        let mut parser = parser::Parser::new(tokens);
        let ast = parser.parse_expression();
        assert_debug_snapshot!(ast);
    }};
}

#[test]
fn expression() {
    test_expression_from_file!("parser/expression.lox");
}
