#[derive(Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    line: u64,
}

impl Token {
    pub fn new(ttype: TokenType, line: u64) -> Token {
        Token { ttype, line }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword(Keywords),
    Literal(Literal),
    Operator(Operator),
    Id(String),
    Line,
}

#[derive(Debug, PartialEq)]
pub enum Keywords {
    For,
    Fn,
    Type,
    If,
    Else,
    Return,
    Continue,
    Break,
    In,
    Mut,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    FloatLit(f64),
    StringLit(String),
    IntLit(i64),
}

#[derive(PartialEq, Debug)]
pub enum Operator {
    Equals,
    Unequal,
    GtEq,
    StEq,
    Greater,
    Smaller,

    Declare,

    LCurly,
    RCurly,
    LBrace,
    RBRace,
    RSquare,
    LSquare,

    Plus,
    Minus,
    Mult,
    Div,
    Modulo,
    LShift,
    RShift,

    Assign,
    PlusAssign,
    MultAssign,
    MinusAssign,
    DivAssign,

    Colon,
    Comma,
    Tag,
    FieldAccess,
    Path,
    Ignore,

    Option,
    DOption,
    Error,
    DError,
}
