#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub ty: TokenType,
    line: u64,
}

impl Token {
    pub fn new(ty: TokenType, line: u64) -> Token {
        Token { ty, line }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Keyword(Keywords),
    Literal(Literal),
    Operator(Operator),
    Id(String),
    Line,
}

#[derive(Debug, PartialEq, Clone)]
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
    Global,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    FloatLit(f64),
    StringLit(String),
    IntLit(i64),
}

#[derive(PartialEq, Debug, Clone)]
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
