use super::{FromJson, JsonError, Parser};

impl FromJson for bool {
    fn parse_json(parser: &mut Parser<'_>) -> Result<Self, JsonError> {
        if parser.consume_literal("true") {
            Ok(true)
        } else if parser.consume_literal("false") {
            Ok(false)
        } else {
            Err(JsonError::ExpectedBool)
        }
    }
}

macro_rules! impl_number_from_json {
    ($($ty:ty),* $(,)?) => {
        $(
            impl FromJson for $ty {
                fn parse_json(parser: &mut Parser<'_>) -> Result<Self, JsonError> {
                    let literal = parser.read_number_literal()?;

                    literal
                        .parse::<$ty>()
                        .map_err(|_| JsonError::InvalidNumber)
                }
            }
        )*
    };
}

impl_number_from_json!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
);
