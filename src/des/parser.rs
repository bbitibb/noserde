use super::JsonError;

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\n' | '\r' | '\t' => {
                    self.consume_char();
                }
                _ => break,
            }
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub fn consume_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    pub fn expect_char(&mut self, expected: char) -> Result<(), JsonError> {
        self.skip_whitespace();

        match self.consume_char() {
            Some(ch) if ch == expected => Ok(()),
            Some(_) => Err(JsonError::ExpectedChar(expected)),
            None => Err(JsonError::UnexpectedEof),
        }
    }

    pub fn consume_if(&mut self, expected: char) -> bool {
        self.skip_whitespace();

        if self.peek_char() == Some(expected) {
            self.consume_char();
            true
        } else {
            false
        }
    }

    pub fn consume_literal(&mut self, literal: &str) -> bool {
        self.skip_whitespace();

        if self.input[self.pos..].starts_with(literal) {
            self.pos += literal.len();
            true
        } else {
            false
        }
    }

    pub fn read_number_literal(&mut self) -> Result<&'a str, JsonError> {
        self.skip_whitespace();

        let start = self.pos;

        if self.peek_char() == Some('-') {
            self.consume_char();
        }

        let mut digit_count = 0;

        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                self.consume_char();
                digit_count += 1;
            } else {
                break;
            }
        }

        if digit_count == 0 {
            return Err(JsonError::ExpectedNumber);
        }

        if self.peek_char() == Some('.') {
            self.consume_char();

            let mut fraction_digit_count = 0;

            while let Some(ch) = self.peek_char() {
                if ch.is_ascii_digit() {
                    self.consume_char();
                    fraction_digit_count += 1;
                } else {
                    break;
                }
            }

            if fraction_digit_count == 0 {
                return Err(JsonError::InvalidNumber);
            }
        }

        if matches!(self.peek_char(), Some('e') | Some('E')) {
            self.consume_char();

            if matches!(self.peek_char(), Some('+') | Some('-')) {
                self.consume_char();
            }

            let mut exponent_digit_count = 0;

            while let Some(ch) = self.peek_char() {
                if ch.is_ascii_digit() {
                    self.consume_char();
                    exponent_digit_count += 1;
                } else {
                    break;
                }
            }

            if exponent_digit_count == 0 {
                return Err(JsonError::InvalidNumber);
            }
        }

        Ok(&self.input[start..self.pos])
    }
}
