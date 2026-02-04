use std::fs::File;

use crate::lexer::{
    lexer::{LexerError, lex},
    tokens::*,
};

fn run_lexer(file: File, expected: Result<Vec<Token>, LexerError>) {
    match (lex(file), expected) {
        (Err(e), Err(e2)) => assert_eq!(e, e2),
        (Err(e), Ok(tokens)) => panic!("Got error {}, when wanted tokens: {:?}", e, tokens),
        (Ok(tokens), Err(e)) => panic!("Wanted error {}, got tokens: {:?}", e, tokens),
        (Ok(tokens), Ok(mut wants)) => {
            //Want to pop front
            wants.reverse();
            println!("Got: {:?}", tokens);
            for t in tokens {
                let want = wants.pop().expect(format!("Next token: {:?}", t).as_str());
                assert_eq!(t, want, "Got: {:?}, Want: {:?}", t, want);
            }
        }
    }
}

#[test]
fn hello_world() {
    let file = File::open("./src/lexer/tests/snippets/hello_world.str").unwrap();
    let expected = vec![
        Token::new(TokenType::Operator(Operator::Tag), 1),
        Token::new(TokenType::Id("pub".to_string()), 1),
        Token::new(TokenType::Operator(Operator::Tag), 1),
        Token::new(TokenType::Id("impure".to_string()), 1),
        Token::new(TokenType::Line, 1),
        Token::new(TokenType::Keyword(Keywords::Fn), 2),
        Token::new(TokenType::Id("main".to_string()), 2),
        Token::new(TokenType::Operator(Operator::LBrace), 2),
        Token::new(TokenType::Operator(Operator::RBRace), 2),
        Token::new(TokenType::Operator(Operator::LCurly), 2),
        Token::new(TokenType::Line, 2),
        Token::new(TokenType::Id("println".to_string()), 3),
        Token::new(TokenType::Operator(Operator::LBrace), 3),
        Token::new(
            TokenType::Literal(Literal::StringLit("Hello World!".to_string())),
            3,
        ),
        Token::new(TokenType::Operator(Operator::RBRace), 3),
        Token::new(TokenType::Line, 3),
        Token::new(TokenType::Operator(Operator::RCurly), 4),
    ];
    run_lexer(file, Ok(expected));
}

#[test]
fn comments() {
    let file = File::open("./src/lexer/tests/snippets/comments.str").unwrap();
    let expected = vec![
        Token::new(TokenType::Line, 1),
        Token::new(TokenType::Keyword(Keywords::If), 2),
        Token::new(TokenType::Line, 2),
        Token::new(TokenType::Line, 3),
        Token::new(TokenType::Keyword(Keywords::If), 4),
        Token::new(TokenType::Line, 4),
        Token::new(TokenType::Line, 5),
        Token::new(TokenType::Keyword(Keywords::Fn), 6),
        Token::new(TokenType::Line, 6),
    ];
    run_lexer(file, Ok(expected));
}

#[test]
fn floats() {
    let file = File::open("./src/lexer/tests/snippets/floats.str").unwrap();
    let expected = vec![
        Token::new(TokenType::Literal(Literal::FloatLit(123.456)), 1),
        Token::new(TokenType::Line, 1),
        Token::new(TokenType::Literal(Literal::IntLit(1)), 2),
        Token::new(TokenType::Literal(Literal::FloatLit(123.456)), 2),
        Token::new(TokenType::Literal(Literal::IntLit(1)), 2),
        Token::new(TokenType::Line, 2),
        Token::new(TokenType::Literal(Literal::FloatLit(123.456)), 3),
        Token::new(TokenType::Line, 3),
        Token::new(TokenType::Id("a".to_string()), 4),
        Token::new(TokenType::Operator(Operator::MultAssign), 4),
        Token::new(TokenType::Literal(Literal::FloatLit(123.456)), 4),
        Token::new(TokenType::Operator(Operator::Mult), 4),
        Token::new(TokenType::Literal(Literal::IntLit(2)), 4),
    ];
    run_lexer(file, Ok(expected));
}
