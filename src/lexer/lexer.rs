use crate::lexer::tokens::{self, *};
use std::{fmt::Display, fs::File, io::Read};

#[derive(Debug)]
pub enum LexerError {
    CouldntReadFile,
    UnicodeUnsupported(u64),
    ControllCharacter(u64),
    IncorrectNumberLiteral(u64),
    SymbolNotFound(u64, char),
}

impl std::error::Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CouldntReadFile => write!(f, "Could not open file"),
            Self::UnicodeUnsupported(line) => {
                write!(f, "Unicode currently unsupported. In line: {}", line)
            }
            Self::ControllCharacter(line) => {
                write!(f, "Ascii controll character found. In line: {}", line)
            }
            Self::IncorrectNumberLiteral(line) => {
                write!(f, "Number literal incorrect. In line: {}", line)
            }
            Self::SymbolNotFound(line, sym) => write!(
                f,
                "Symbol not used for any operators. Line: {}, Symbol: {}",
                line, sym
            ),
        }
    }
}

enum LexState {
    Open,
    Id(String),
    IntLit(String),
    FloatLit(String),
    StringLit(String),
    SymbolLit(String),
    LineComment,
    BlockComment(bool),
}

struct StateMachine {
    state: LexState,
    line: u64,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: LexState::Open,
            line: 1,
        }
    }

    #[must_use]
    fn read_string(&mut self, c: char, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        match &mut self.state {
            LexState::Open => self.state = LexState::Id(String::from(c)),
            LexState::Id(s) | LexState::StringLit(s) => s.push(c),
            LexState::LineComment => (),
            LexState::BlockComment(escape) => *escape = false,
            LexState::FloatLit(_) | LexState::IntLit(_) => {
                return Err(LexerError::IncorrectNumberLiteral(self.line));
            }
            LexState::SymbolLit(_) => {
                let sym = std::mem::replace(&mut self.state, LexState::Id(String::from(c)));
                if let LexState::SymbolLit(s) = sym {
                    resolve_sym(s, tokens, self.line)?;
                } else {
                    unreachable!();
                }
            }
        }

        Ok(())
    }

    #[must_use]
    fn read_num(&mut self, c: char, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        match &mut self.state {
            LexState::Open => self.state = LexState::IntLit(String::from(c)),
            LexState::LineComment => (),
            LexState::BlockComment(escape) => *escape = false,
            LexState::IntLit(s) | LexState::FloatLit(s) => s.push(c),
            LexState::Id(s) | LexState::StringLit(s) => s.push(c),
            LexState::SymbolLit(_) => {
                let sym = std::mem::replace(&mut self.state, LexState::IntLit(String::from(c)));
                if let LexState::SymbolLit(s) = sym {
                    match resolve_sym(s, tokens, self.line) {
                        Ok(None) => (),
                        Ok(Some(new_state)) => self.state = new_state,
                        Err(err) => return Err(err),
                    }
                } else {
                    unreachable!();
                }
            }
        }

        Ok(())
    }

    #[must_use]
    fn read_space(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        match std::mem::replace(&mut self.state, LexState::Open) {
            LexState::Open => (),
            LexState::Id(s) => resolve_id(s, tokens, self.line),
            LexState::IntLit(s) => resolve_float(s, tokens, self.line),
            LexState::FloatLit(s) => resolve_float(s, tokens, self.line),
            LexState::SymbolLit(sym) => match resolve_sym(sym, tokens, self.line) {
                Ok(None) => (),
                Ok(Some(new_state)) => self.state = new_state,
                Err(err) => return Err(err),
            },
            LexState::LineComment => self.state = LexState::LineComment,
            LexState::BlockComment(_) => self.state = LexState::BlockComment(false),
            LexState::StringLit(mut s) => {
                s.push(' ');
                self.state = LexState::StringLit(s)
            }
        }
        Ok(())
    }

    #[must_use]
    fn read_new_line(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        self.read_space(tokens)?;
        if matches!(self.state, LexState::LineComment) {
            self.state = LexState::Open;
        }
        tokens.push(Token::new(TokenType::Line, self.line));
        self.line += 1;
        Ok(())
    }

    #[must_use]
    fn read_sym(&mut self, c: char, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        match std::mem::replace(&mut self.state, LexState::SymbolLit(String::from(c))) {
            LexState::Open => (),
            LexState::IntLit(mut s) => {
                if c == '.' {
                    s.push(c);
                    self.state = LexState::FloatLit(s.clone());
                } else {
                    resolve_int(s, tokens, self.line);
                }
            }
            LexState::StringLit(mut s) => {
                if c == '\"' {
                    resolve_string(s, tokens, self.line);
                    self.state = LexState::Open;
                } else {
                    s.push(c);
                }
            }
            LexState::FloatLit(s) => resolve_float(s, tokens, self.line),
            LexState::Id(s) => resolve_id(s, tokens, self.line),
            LexState::LineComment => {
                self.state = LexState::LineComment;
            }
            LexState::BlockComment(mut escape) => {
                if escape && c == '\\' {
                    self.state = LexState::Open;
                } else if c == '*' {
                    escape = true;
                } else {
                    escape = false;
                }
                self.state = LexState::BlockComment(escape);
            }
            LexState::SymbolLit(mut sym) => {
                if c == '\"' {
                    resolve_sym(sym, tokens, self.line)?;
                    self.state = LexState::StringLit(String::new());
                    return Ok(());
                }
                sym.push(c);
                self.state = LexState::SymbolLit(sym);
            }
        }
        Ok(())
    }
}

pub fn lex(mut file: File) -> Result<Vec<Token>, LexerError> {
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|_e| LexerError::CouldntReadFile)?;

    let mut tokens = Vec::new();
    let mut state = StateMachine::new();

    for char in content.chars() {
        if !char.is_ascii() {
            return Err(LexerError::UnicodeUnsupported(state.line));
        }

        match char {
            char if char.is_ascii_control() => {
                return Err(LexerError::ControllCharacter(state.line));
            }

            char if char.is_ascii_alphabetic() => {
                state.read_string(char.to_ascii_lowercase(), &mut tokens)?;
            }

            char if char.is_ascii_digit() => {
                state.read_num(char, &mut tokens)?;
            }

            char if char == ' ' => {
                state.read_space(&mut tokens)?;
            }

            char if char == '\n' => {
                state.read_new_line(&mut tokens)?;
            }

            char if char.is_ascii_punctuation() => {
                state.read_sym(char, &mut tokens)?;
            }

            _ => {
                panic!("Ascii parser not working: {}, {:?}", char, char);
            }
        }
    }

    Ok(tokens)
}

fn resolve_id(str: String, tokens: &mut Vec<Token>, line: u64) {
    match tokens::Keywords::try_from_string(&str) {
        Some(t) => tokens.push(Token::new(TokenType::Keyword(t), line)),
        None => tokens.push(Token::new(TokenType::Id(str), line)),
    }
}

fn resolve_sym(
    str: String,
    tokens: &mut Vec<Token>,
    line: u64,
) -> Result<Option<LexState>, LexerError> {
    // Since there is no case were the first 2 symbols form a double symbol and
    // second and third do as well we can do this greedily
    let mut operators = Vec::new();
    let double_op = |last, replace, ops: &mut Vec<Operator>| {
        if ops.last() == Some(&last) {
            ops.pop();
            replace
        } else {
            last
        }
    };

    for sym in str.chars() {
        let op = match sym {
            '*' => {
                // We need to check if we have started a block comment (using /*)
                let last = operators.last();
                if last == Some(&Operator::Div) {
                    // Yes i know the block comment could be escaped in the same symbol string
                    // like /**/. Idc :)
                    return Ok(Some(LexState::BlockComment(false)));
                }
                Operator::Mult
            }
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            '%' => Operator::Modulo,

            '/' => {
                // We need to check if we have started a line comment (using //)
                let last = operators.last();
                if last == Some(&Operator::Div) {
                    return Ok(Some(LexState::LineComment));
                }
                Operator::Div
            }

            '(' => Operator::LBrace,
            ')' => Operator::RBRace,
            '[' => Operator::LSquare,
            ']' => Operator::RSquare,
            '{' => Operator::LCurly,
            '}' => Operator::RCurly,

            '>' => double_op(Operator::Greater, Operator::RShift, &mut operators),
            '<' => double_op(Operator::Smaller, Operator::LShift, &mut operators),

            ',' => Operator::Comma,
            '\'' => Operator::Tag,
            '.' => Operator::FieldAccess,
            '_' => Operator::Ignore,
            ':' => double_op(Operator::Colon, Operator::Path, &mut operators),

            '?' => double_op(Operator::Option, Operator::DOption, &mut operators),
            '!' => double_op(Operator::Error, Operator::DError, &mut operators),

            '=' => match operators.pop() {
                None => Operator::Assign,
                Some(last) => match last {
                    Operator::Assign => Operator::Equals,

                    Operator::Plus => Operator::PlusAssign,
                    Operator::Mult => Operator::MultAssign,
                    Operator::Minus => Operator::MinusAssign,
                    Operator::Div => Operator::DivAssign,

                    Operator::Greater => Operator::GtEq,
                    Operator::Smaller => Operator::StEq,

                    // !=
                    Operator::Error => Operator::Unequal,

                    // :=
                    Operator::Colon => Operator::Declare,
                    _ => {
                        operators.push(last);
                        Operator::Assign
                    }
                },
            },

            '\"' => panic!(
                "Compiler Error: Quote mark made it into resolve_symbol. Line: {}",
                line
            ),

            _ => return Err(LexerError::SymbolNotFound(line, sym)),
        };

        operators.push(op);
    }

    for op in operators {
        tokens.push(Token::new(TokenType::Operator(op), line));
    }
    Ok(None)
}

fn resolve_float(str: String, tokens: &mut Vec<Token>, line: u64) {
    let f: f64 = str
        .parse()
        .expect("Faulty parse from float string to float");
    tokens.push(Token::new(TokenType::Literal(Literal::FloatLit(f)), line));
}

fn resolve_int(str: String, tokens: &mut Vec<Token>, line: u64) {
    let i: i64 = str.parse().expect("Faulty parse from int string to int");
    tokens.push(Token::new(TokenType::Literal(Literal::IntLit(i)), line));
}

fn resolve_string(str: String, tokens: &mut Vec<Token>, line: u64) {
    tokens.push(Token::new(
        TokenType::Literal(Literal::StringLit(str)),
        line,
    ));
}
