use insta::assert_debug_snapshot;
use lory::lexer;

#[test]
fn identifiers() {
    let tokens = lexer::tokenize(include_str!("lexer/identifiers.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}

#[test]
fn keywords() {
    let tokens = lexer::tokenize(include_str!("lexer/keywords.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}

#[test]
fn numbers() {
    let tokens = lexer::tokenize(include_str!("lexer/numbers.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}

#[test]
fn punctuators() {
    let tokens = lexer::tokenize(include_str!("lexer/punctuators.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}

#[test]
fn strings() {
    let tokens = lexer::tokenize(include_str!("lexer/strings.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}

#[test]
fn whitespace() {
    let tokens = lexer::tokenize(include_str!("lexer/whitespace.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}

#[test]
fn comments() {
    let tokens = lexer::tokenize(include_str!("lexer/comments.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}

#[test]
fn unexpectedchar() {
    let tokens = lexer::tokenize(include_str!("lexer/unexpectedchar.lox"));
    assert_debug_snapshot!(tokens.collect::<Vec<_>>());
}
