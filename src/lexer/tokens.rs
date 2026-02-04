#[derive(Debug, PartialEq)]
pub struct Token {
    ttype: TokenType,
    line: u64,
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

impl Keywords {
    pub fn try_from_string(str: &String) -> Option<Self> {
        match str.as_str() {
            "for" => Some(Self::For),
            "fn" => Some(Self::Fn),
            "type" => Some(Self::Type),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "continue" => Some(Self::Continue),
            "break" => Some(Self::Break),
            "return" => Some(Self::Return),
            "in" => Some(Self::In),
            "mut" => Some(Self::Mut),
            _ => None,
        }
    }
}

impl Token {
    pub fn new(ttype: TokenType, line: u64) -> Token {
        Token { ttype, line }
    }
}
