use super::{JsonError, Parser};

pub trait FromJson: Sized {
    fn parse_json(parser: &mut Parser<'_>) -> Result<Self, JsonError>;

    fn from_json(input: &str) -> Result<Self, JsonError> {
        let mut parser = Parser::new(input);

        let value = Self::parse_json(&mut parser)?;

        parser.skip_whitespace();

        if !parser.is_eof() {
            return Err(JsonError::TrailingCharacters);
        }

        Ok(value)
    }
}