pub struct Token {
    ttype: TokenType,
    line: u64,
}

pub enum TokenType {
    Keyword(Keywords),
    Literal(Literal),
    Operator(Operator),
    Id(String),
    Line,
}

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

pub enum Literal {}

pub enum Operator {}

impl Keywords {
    pub fn try_from_string(str: &String) -> Option<Self> {
        match str.as_str() {
            "for" => Some(Self::For),
            "fn" => Some(Self::Fn),
            "type" => Some(Self::Type),
            "if" => Some(Self::Else),
            "return" => Some(Self::Return),
            _ => None,
        }
    }
}

impl Token {
    pub fn new(ttype: TokenType, line: u64) -> Token {
        Token { ttype, line }
    }
}
