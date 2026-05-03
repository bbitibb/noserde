use super::{FromJson, JsonError, Parser};

impl FromJson for String {
    fn parse_json(parser: &mut Parser<'_>) -> Result<Self, JsonError> {
        parser.skip_whitespace();

        if parser.consume_char() != Some('"') {
            return Err(JsonError::ExpectedString);
        }

        let mut result = String::new();

        loop {
            let ch = parser.consume_char().ok_or(JsonError::UnexpectedEof)?;

            match ch {
                '"' => return Ok(result),

                '\\' => {
                    let escaped = parser.consume_char().ok_or(JsonError::UnexpectedEof)?;

                    match escaped {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'b' => result.push('\u{08}'),
                        'f' => result.push('\u{0C}'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        'u' => {
                            let code = parse_hex4(parser)?;
                            let ch = char::from_u32(code)
                                .ok_or(JsonError::InvalidUnicodeEscape)?;

                            result.push(ch);
                        }
                        other => return Err(JsonError::InvalidEscape(other)),
                    }
                }

                ch if ch.is_control() => {
                    return Err(JsonError::UnexpectedChar(ch));
                }

                other => result.push(other),
            }
        }
    }
}

fn parse_hex4(parser: &mut Parser<'_>) -> Result<u32, JsonError> {
    let mut value = 0;

    for _ in 0..4 {
        let ch = parser.consume_char().ok_or(JsonError::UnexpectedEof)?;
        let digit = ch.to_digit(16).ok_or(JsonError::InvalidUnicodeEscape)?;

        value = value * 16 + digit;
    }

    Ok(value)
}