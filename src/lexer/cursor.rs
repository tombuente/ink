use std::{iter::Peekable, str::Chars};

pub struct Cursor<'a> {
    pub iter: Peekable<Chars<'a>>,
    pos: usize,
    peek_pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
            pos: 0,
            peek_pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn peek_pos(&self) -> usize {
        self.peek_pos
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.iter.next() {
            self.pos = self.peek_pos;
            self.peek_pos += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }
}
