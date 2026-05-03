use super::{FromJson, JsonError, Parser};

impl<T: FromJson> FromJson for Option<T> {
    fn parse_json(parser: &mut Parser<'_>) -> Result<Self, JsonError> {
        if parser.consume_literal("null") {
            Ok(None)
        } else {
            Ok(Some(T::parse_json(parser)?))
        }
    }
}

impl<T: FromJson> FromJson for Vec<T> {
    fn parse_json(parser: &mut Parser<'_>) -> Result<Self, JsonError> {
        parser.expect_char('[')?;

        let mut values = Vec::new();

        if parser.consume_if(']') {
            return Ok(values);
        }

        loop {
            let value = T::parse_json(parser)?;
            values.push(value);

            if parser.consume_if(',') {
                continue;
            }

            parser.expect_char(']')?;
            break;
        }

        Ok(values)
    }
}