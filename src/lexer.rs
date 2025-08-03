use std::iter;

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Asterix,
    Slash,
    Assign,
    Equal,
    Let,
    Ident(String),
}

#[derive(Debug, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    UnexpectedChar(char),
}

#[derive(Debug, PartialEq)]
pub struct SpannedLexError {
    pub error: LexError,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

#[allow(dead_code)]
pub fn tokenize(input: &str) -> Result<Vec<SpannedToken>, SpannedLexError> {
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
                    _ => tokens.push(SpannedToken {
                        token: Token::Ident(word),
                        span,
                    }),
                }
            }
            _ => {
                return Err(SpannedLexError {
                    error: LexError::UnexpectedChar(ch),
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
    fn test_tokenize() {
        let input = "+-@";
        // let positions: Vec<usize> = input.char_indices().map(|(i, _)| i).collect();
        tokenize(input).unwrap();
    }
}
