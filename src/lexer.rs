use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Arithmetic operators
    Plus,
    Minus,
    Asterix,
    Slash,

    // Assignment and comparison operators
    Less,
    Greater,
    Assign,
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
    Ident(String),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

#[allow(dead_code)]
pub fn tokenize(input: &str) -> Result<Vec<SpannedToken>, SpannedError> {
    let mut tokens = Vec::new();
    let mut cursor = input.char_indices().peekable();

    while let Some((pos, ch)) = cursor.next() {
        match ch {
            _ if ch.is_whitespace() => continue,
            '+' => tokens.push(SpannedToken {
                token: Token::Plus,
                span: Span::from_char(pos, ch),
            }),
            '-' => tokens.push(SpannedToken {
                token: Token::Minus,
                span: Span::from_char(pos, ch),
            }),
            '*' => tokens.push(SpannedToken {
                token: Token::Asterix,
                span: Span::from_char(pos, ch),
            }),
            '/' => tokens.push(SpannedToken {
                token: Token::Slash,
                span: Span::from_char(pos, ch),
            }),
            '<' => tokens.push(SpannedToken {
                token: Token::Less,
                span: Span::from_char(pos, ch),
            }),
            '>' => tokens.push(SpannedToken {
                token: Token::Greater,
                span: Span::from_char(pos, ch),
            }),
            '=' => {
                if let Some((ch_pos, ch)) = cursor.next_if(|&(_, ch)| ch == '=') {
                    tokens.push(SpannedToken {
                        token: Token::Equal,
                        span: Span::from_chars(pos, ch_pos, ch),
                    });
                } else {
                    tokens.push(SpannedToken {
                        token: Token::Assign,
                        span: Span::from_char(pos, ch),
                    })
                }
            }
            '!' => {
                if let Some((ch_pos, ch)) = cursor.next_if(|&(_, ch)| ch == '=') {
                    tokens.push(SpannedToken {
                        token: Token::NotEqual,
                        span: Span::from_chars(pos, ch_pos, ch),
                    });
                } else {
                    tokens.push(SpannedToken {
                        token: Token::Bang,
                        span: Span::from_char(pos, ch),
                    })
                }
            }
            '(' => tokens.push(SpannedToken {
                token: Token::LeftParan,
                span: Span::from_char(pos, ch),
            }),
            ')' => tokens.push(SpannedToken {
                token: Token::RightParan,
                span: Span::from_char(pos, ch),
            }),
            '{' => tokens.push(SpannedToken {
                token: Token::LeftBrace,
                span: Span::from_char(pos, ch),
            }),
            '}' => tokens.push(SpannedToken {
                token: Token::RightBrace,
                span: Span::from_char(pos, ch),
            }),
            ',' => tokens.push(SpannedToken {
                token: Token::Comma,
                span: Span::from_char(pos, ch),
            }),
            ';' => tokens.push(SpannedToken {
                token: Token::Semicolon,
                span: Span::from_char(pos, ch),
            }),
            'a'..='z' => {
                let mut word = String::new();
                word.push(ch);
                word.extend(
                    cursor
                        .by_ref()
                        .take_while(|(_, c)| c.is_alphanumeric())
                        .map(|(_, c)| c),
                );

                let span = Span::from_str(pos, word.as_str());

                match word.as_str() {
                    "let" => tokens.push(SpannedToken {
                        token: Token::Let,
                        span,
                    }),
                    "return" => tokens.push(SpannedToken {
                        token: Token::Return,
                        span,
                    }),
                    "if" => tokens.push(SpannedToken {
                        token: Token::If,
                        span,
                    }),
                    "fn" => tokens.push(SpannedToken {
                        token: Token::Fn,
                        span,
                    }),
                    "true" => tokens.push(SpannedToken {
                        token: Token::Bool(true),
                        span,
                    }),
                    "false" => tokens.push(SpannedToken {
                        token: Token::Bool(false),
                        span,
                    }),
                    _ => tokens.push(SpannedToken {
                        token: Token::Ident(word),
                        span,
                    }),
                }
            }
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

impl Span {
    fn from_char(offset: usize, ch: char) -> Self {
        Self {
            start: offset,
            end: offset + ch.len_utf8(),
        }
    }

    fn from_chars(offset: usize, ch_offset: usize, ch: char) -> Self {
        Self {
            start: offset,
            end: ch_offset + ch.len_utf8(),
        }
    }

    fn from_str(offset: usize, str: &str) -> Self {
        Self {
            start: offset,
            end: offset + str.len(),
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
        < > = == ! !=
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
            Token::Ident("word".to_string()),
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
