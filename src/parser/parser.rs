use crate::{
    lexer::tokens::*,
    parser::ast::{self, Program},
};

use chumsky::prelude::*;

pub fn parse(tokens: Vec<Token>) -> ParseResult<Program, EmptyErr> {
    gen_parser().parse(&tokens)
}

fn gen_parser<'a>() -> impl Parser<'a, &'a [Token], Program> {
    let pure_type = select!(Token {
        ty: TokenType::Id(i),
        ..
    });
    let exp;
    let func;
    let global = select!(Token {
        ty: TokenType::Keyword(Keywords::Global),
        ..
    })
    .then(select! {Token {
            ty: TokenType::Id(i), ..
        }
    })
    .then(select!(Token {
        ty: TokenType::Operator(Operator::Equals),
        ..
    }))
    .then(exp);

    let items = choice((global, func));
    let program = items.repeated();
    program
}
