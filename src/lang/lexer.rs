#![allow(dead_code)]
use anyhow::Result;
use std::str::from_utf8;
use std::{
    cell::RefCell,
    error::Error,
    io::{BufRead, ErrorKind},
};

type TokResult = Result<Option<Token>, Box<dyn Error>>;

#[derive(Debug, PartialEq, Clone)]
enum Lexeme {
    GroupBegin,
    GroupEnd,
    FunctionBegin,
    FunctionEnd,
    Variable,
    String(String),
    Indent,
    Newline,
}

impl Lexeme {
    fn from(token: &str) -> Self {
        use Lexeme::*;
        match token {
            "{" => GroupBegin,
            "}" => GroupEnd,
            "[" => FunctionBegin,
            "]" => FunctionEnd,
            "\n" => Newline,
            "\t" => Indent,
            "%" => Variable,
            v => String(v.into()),
        }
    }

    fn compliments(&self) -> Option<Self> {
        use Lexeme::*;
        match *self {
            GroupBegin => Some(GroupEnd),
            FunctionBegin => Some(FunctionEnd),
            Variable => Some(Variable),
            _ => None,
        }
    }
}

// A basic struct providing the position of the lexer or
// a lexeme.
#[derive(Clone, Debug, Copy, Default)]
struct Position {
    logical: (u16, u16),
    index: u16,
}

impl Position {
    fn increment(&mut self) {
        self.logical.1 += 1;
        self.index += 1;
    }

    fn newline(&mut self) {
        self.increment();
        self.logical.0 += 1;
        self.logical.1 = 0;
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    lexeme: Lexeme,
    positions: Option<(RefCell<Position>, RefCell<Position>)>,
}

pub struct Lexer<T: BufRead> {
    position: RefCell<Position>,
    reader: T,
    tokens: Vec<Token>,
}

impl<T: BufRead> Lexer<T> {
    pub fn from(reader: T) -> Self {
        Self {
            reader,
            position: RefCell::new(Position::default()),
            tokens: Vec::default(),
        }
    }

    pub fn next_token(&mut self) -> TokResult {
        let mut buf: [u8; 1] = [0; 1];

        match self.reader.read_exact(&mut buf) {
            Ok(_) => {
                let mut lexeme = Lexeme::from(from_utf8(&buf)?);
                let position = self.position.clone();
                self.position.borrow_mut().increment();
                match lexeme {
                    Lexeme::String(ref mut val) => {
                        loop {
                            // NOTE: This may not work for multibyte characters, this is just a guess so
                            // I'm not for certain.
                            let internal_buffer = String::from(from_utf8(self.reader.fill_buf()?)?);
                            if internal_buffer.len() == 0 {
                                break;
                            };

                            let first = internal_buffer.chars().next().unwrap().to_string();
                            match Lexeme::from(first.as_str()) {
                                Lexeme::String(v) => {
                                    val.push_str(&v);
                                    self.reader.consume(v.len());
                                    self.position.borrow_mut().increment();
                                }
                                _ => break,
                            }
                        }

                        Ok(Some(Token {
                            lexeme,
                            positions: Some((position, self.position.clone())),
                        }))
                    }
                    Lexeme::Newline => {
                        self.position.borrow_mut().newline();
                        Ok(Some(Token {
                            lexeme,
                            positions: Some((position, self.position.clone())),
                        }))
                    }
                    _ => Ok(Some(Token {
                        lexeme,
                        positions: Some((position, self.position.clone())),
                    })),
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => Ok(None),
                _ => Err(Box::new(err)),
            },
        }
    }
}
