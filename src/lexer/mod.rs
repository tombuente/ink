mod cursor;

use cursor::Cursor;

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub struct LocatedToken {
    token: Token,
    location: usize,
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    IllegalToken,
}

#[derive(Debug, PartialEq)]
pub struct LocatedLexError {
    pub error: LexError,
    pub location: usize,
}

pub type LexResult = Result<LocatedToken, LocatedLexError>;

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.cursor.next() {
            match ch {
                '+' => Some(Ok(LocatedToken {
                    token: Token::Plus,
                    location: self.cursor.pos(),
                })),
                '-' => Some(Ok(LocatedToken {
                    token: Token::Minus,
                    location: self.cursor.pos(),
                })),
                _ => Some(Err(LocatedLexError {
                    error: LexError::IllegalToken,
                    location: self.cursor.pos(),
                })),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "+-@";
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();

        let expected = vec![
            Ok(LocatedToken {
                token: Token::Plus,
                location: 0,
            }),
            Ok(LocatedToken {
                token: Token::Minus,
                location: 1,
            }),
            Err(LocatedLexError {
                error: LexError::IllegalToken,
                location: 2,
            }),
        ];

        assert_eq!(tokens, expected);
    }
}
