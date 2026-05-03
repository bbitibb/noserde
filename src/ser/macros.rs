#[macro_export]
macro_rules! impl_to_json {
    (
        $name:ident {
            $($field:ident),* $(,)?
        }
    ) => {
        impl $crate::ToJson for $name {
            fn write_json(&self, out: &mut String) {
                out.push('{');

                let mut first = true;

                $(
                    if !first {
                        out.push(',');
                    }

                    first = false;

                    out.push('"');
                    out.push_str(stringify!($field));
                    out.push_str("\":");

                    self.$field.write_json(out);
                )*

                out.push('}');
            }
        }
    };
}

#[macro_export]
macro_rules! json_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $field_vis:vis $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $(
                $field_vis $field: $ty,
            )*
        }

        $crate::impl_json! {
            $name {
                $($field : $ty),*
            }
        }
    };
}