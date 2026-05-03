#[macro_export]
macro_rules! impl_from_json {
    (
        $name:ident {
            $(
                $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        impl $crate::FromJson for $name {
            fn parse_json(
                parser: &mut $crate::des::Parser<'_>
            ) -> Result<Self, $crate::JsonError> {
                parser.expect_char('{')?;

                $(
                    let mut $field: Option<$ty> = None;
                )*

                if parser.consume_if('}') {
                    return Ok(Self {
                        $(
                            $field: $field.ok_or(
                                $crate::JsonError::MissingField(stringify!($field))
                            )?,
                        )*
                    });
                }

                loop {
                    let key = <String as $crate::FromJson>::parse_json(parser)?;

                    parser.expect_char(':')?;

                    match key.as_str() {
                        $(
                            stringify!($field) => {
                                $field = Some(
                                    <$ty as $crate::FromJson>::parse_json(parser)?
                                );
                            }
                        )*
                        _ => return Err($crate::JsonError::UnknownField(key)),
                    }

                    if parser.consume_if(',') {
                        continue;
                    }

                    parser.expect_char('}')?;
                    break;
                }

                Ok(Self {
                    $(
                        $field: $field.ok_or(
                            $crate::JsonError::MissingField(stringify!($field))
                        )?,
                    )*
                })
            }
        }
    };
}

#[macro_export]
macro_rules! impl_json {
    (
        $name:ident {
            $(
                $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $crate::impl_to_json! {
            $name {
                $($field),*
            }
        }

        $crate::impl_from_json! {
            $name {
                $($field : $ty),*
            }
        }
    };
}