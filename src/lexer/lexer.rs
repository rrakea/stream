use crate::lexer::tokens::{self, *};
use std::{fmt::Display, fs::File, io::Read};

#[derive(Debug)]
enum LexerError {
    CouldntReadFile,
    UnicodeUnsupported(u64),
    ControllCharacter(u64),
    IncorrectNumberLiteral(u64),
}

impl std::error::Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
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
                    resolve_sym(s, tokens, self.line);
                } else {
                    unreachable!();
                }
            }
        }

        Ok(())
    }

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
                    resolve_sym(s, tokens, self.line);
                } else {
                    unreachable!();
                }
            }
        }

        Ok(())
    }

    fn read_space(&mut self, tokens: &mut Vec<Token>) {
        match std::mem::replace(&mut self.state, LexState::Open) {
            LexState::Open => (),
            LexState::Id(s) => resolve_id(s, tokens, self.line),
            LexState::IntLit(s) => resolve_float(s, tokens, self.line),
            LexState::FloatLit(s) => resolve_float(s, tokens, self.line),
            LexState::SymbolLit(sym) => resolve_sym(sym, tokens, self.line),
            LexState::LineComment => self.state = LexState::LineComment,
            LexState::BlockComment(_) => self.state = LexState::BlockComment(false),
            LexState::StringLit(mut s) => {
                s.push(' ');
                self.state = LexState::StringLit(s)
            }
        }
    }

    fn read_new_line(&mut self, tokens: &mut Vec<Token>) {
        self.read_space(tokens);
        if matches!(self.state, LexState::LineComment) {
            self.state = LexState::Open;
        }
        tokens.push(Token::new(TokenType::Line, self.line));
        self.line += 1
    }

    fn read_sym(&mut self, c: char, tokens: &mut Vec<Token>) {
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
                sym.push(c);
                self.state = LexState::SymbolLit(sym);
            }
        }
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
                state.read_string(char.to_ascii_lowercase(), &mut tokens);
            }

            char if char.is_ascii_digit() => {
                state.read_num(char, &mut tokens);
            }

            char if char == ' ' => {
                state.read_space(&mut tokens);
            }

            char if char == '\n' => {
                state.read_new_line(&mut tokens);
            }

            char if char.is_ascii_punctuation() => {
                state.read_sym(char, &mut tokens);
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

fn resolve_sym(str: String, tokens: &mut Vec<Token>, line: u64) {}

fn resolve_float(str: String, tokens: &mut Vec<Token>, line: u64) {}

fn resolve_int(str: String, tokens: &mut Vec<Token>, line: u64) {}

fn resolve_string(str: String, tokens: &mut Vec<Token>, line: u64) {}
