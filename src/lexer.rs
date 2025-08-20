use std::{iter::Peekable, str::CharIndices};

use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Arithmetic
    Plus,
    Minus,
    Asterix,
    Slash,

    // Assignment and comparison
    Assign,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    Bang,
    NotEqual,

    // Delimiters
    LeftParan,
    RightParan,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    // Keywords and identifiers
    Let,
    Return,
    If,
    Fn,
    Bool(bool),
    Word(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("unexpected character: '{0}'")]
    UnexpectedChar(char),
}

#[derive(Debug, Error, PartialEq)]
#[error("{error} at {span:?}")]
pub struct SpannedError {
    pub error: Error,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

type Cursor<'a> = Peekable<CharIndices<'a>>;

pub fn tokenize(input: &str) -> Result<Vec<SpannedToken>, SpannedError> {
    let mut tokens = Vec::new();
    let mut cursor = input.char_indices().peekable();

    while let Some((pos, ch)) = cursor.next() {
        match ch {
            _ if ch.is_whitespace() => continue,
            '+' => tokens.push(SpannedToken::from_char(Token::Plus, pos, ch)),
            '-' => tokens.push(SpannedToken::from_char(Token::Minus, pos, ch)),
            '*' => tokens.push(SpannedToken::from_char(Token::Asterix, pos, ch)),
            '/' => tokens.push(SpannedToken::from_char(Token::Slash, pos, ch)),
            '<' => tokens.push(compound(
                &mut cursor,
                pos,
                ch,
                '=',
                Token::LessEqual,
                Token::Less,
            )),
            '>' => tokens.push(compound(
                &mut cursor,
                pos,
                ch,
                '=',
                Token::GreaterEqual,
                Token::Greater,
            )),
            '=' => tokens.push(compound(
                &mut cursor,
                pos,
                ch,
                '=',
                Token::Equal,
                Token::Assign,
            )),
            '!' => tokens.push(compound(
                &mut cursor,
                pos,
                ch,
                '=',
                Token::NotEqual,
                Token::Bang,
            )),
            '(' => tokens.push(SpannedToken::from_char(Token::LeftParan, pos, ch)),
            ')' => tokens.push(SpannedToken::from_char(Token::RightParan, pos, ch)),
            '{' => tokens.push(SpannedToken::from_char(Token::LeftBrace, pos, ch)),
            '}' => tokens.push(SpannedToken::from_char(Token::RightBrace, pos, ch)),
            ',' => tokens.push(SpannedToken::from_char(Token::Comma, pos, ch)),
            ';' => tokens.push(SpannedToken::from_char(Token::Semicolon, pos, ch)),
            'a'..='z' => tokens.push(word(&mut cursor, pos, ch)),
            _ => {
                return Err(SpannedError {
                    error: Error::UnexpectedChar(ch),
                    span: Span::from_char(pos, ch),
                });
            }
        }
    }

    Ok(tokens)
}

/// If the next character in the cursor matches `next_ch`, returns the `compound` token
/// spanning both `ch` and `next_ch`. Otherwise, returns the `fallback` token.
fn compound(
    cursor: &mut Cursor<'_>,
    pos: usize,
    ch: char,
    next_ch: char,
    compound: Token,
    fallback: Token,
) -> SpannedToken {
    if let Some((ch_pos, ch)) = cursor.next_if(|&(_, ch)| ch == next_ch) {
        SpannedToken::from_chars(compound, pos, ch_pos, ch)
    } else {
        SpannedToken::from_char(fallback, pos, ch)
    }
}

fn word(cursor: &mut Cursor<'_>, pos: usize, ch: char) -> SpannedToken {
    let mut word = String::new();
    word.push(ch);
    word.extend(
        cursor
            .by_ref()
            .take_while(|(_, ch)| ch.is_alphanumeric())
            .map(|(_, ch)| ch),
    );

    let span = Span::from_str(pos, word.as_str());

    match word.as_str() {
        "let" => SpannedToken {
            token: Token::Let,
            span,
        },
        "return" => SpannedToken {
            token: Token::Return,
            span,
        },
        "if" => SpannedToken {
            token: Token::If,
            span,
        },
        "fn" => SpannedToken {
            token: Token::Fn,
            span,
        },
        "true" => SpannedToken {
            token: Token::Bool(true),
            span,
        },
        "false" => SpannedToken {
            token: Token::Bool(false),
            span,
        },
        _ => SpannedToken {
            token: Token::Word(word),
            span,
        },
    }
}

impl SpannedToken {
    fn from_char(token: Token, pos: usize, ch: char) -> Self {
        Self {
            token,
            span: Span::from_char(pos, ch),
        }
    }

    fn from_chars(token: Token, pos: usize, ch_pos: usize, ch: char) -> Self {
        Self {
            token,
            span: Span::from_chars(pos, ch_pos, ch),
        }
    }
}

impl Span {
    fn from_char(pos: usize, ch: char) -> Self {
        Self {
            start: pos,
            end: pos + ch.len_utf8(),
        }
    }

    fn from_chars(pos: usize, ch_pos: usize, ch: char) -> Self {
        Self {
            start: pos,
            end: ch_pos + ch.len_utf8(),
        }
    }

    fn from_str(pos: usize, str: &str) -> Self {
        Self {
            start: pos,
            end: pos + str.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_tokens() {
        let input = "
        + - * /
        < > <= >= = == ! !=
        ( ) { } , ;
        
        let
        return
        if
        fn
        true
        false
        word";
        let tokens: Vec<_> = tokenize(input)
            .unwrap()
            .into_iter()
            .map(|spanned_token| spanned_token.token)
            .collect();

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterix,
            Token::Slash,
            Token::Less,
            Token::Greater,
            Token::LessEqual,
            Token::GreaterEqual,
            Token::Assign,
            Token::Equal,
            Token::Bang,
            Token::NotEqual,
            Token::LeftParan,
            Token::RightParan,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Let,
            Token::Return,
            Token::If,
            Token::Fn,
            Token::Bool(true),
            Token::Bool(false),
            Token::Word("word".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokensize_spans() {
        let input = "+ let";
        let spans: Vec<_> = tokenize(input)
            .unwrap()
            .into_iter()
            .map(|spanned_token| spanned_token.span)
            .collect();

        let expected = vec![Span { start: 0, end: 1 }, Span { start: 2, end: 5 }];

        assert_eq!(spans, expected)
    }
}
