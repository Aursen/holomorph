extern crate byteorder;
extern crate diesel;

macro_rules! impl_variant {
    ($name: ident, $($field_name: ident | $field_type: ty),*) => {
        #[derive(Clone, Debug)]
        pub enum $name {
            $(
                $field_name($field_type),
            )*
        }

        impl Protocol for $name {
            fn deserialize<R: Read>(rdr: &mut R) -> Result<$name> {
                use $crate::io::ReadExt;
                use std::io::{Error, ErrorKind};
                let id = try!(rdr.read_i16());

                match id {
                    $(
                        _ if id == <$field_type as Protocol>::id() =>
                            Ok($name::$field_name(try!(<$field_type as Protocol>
                                ::deserialize(rdr)))),
                    )*
                    _ => Err(Error::new(ErrorKind::Other, "bad protocol id")),
                }
            }

            fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
                use $crate::io::WriteExt;
                match *self {
                    $(
                        $name::$field_name(ref val) => {
                            try!(wtr.write_i16(<$field_type as Protocol>::id()));
                            val.serialize(wtr)
                        }
                    )*
                }
            }

            fn id() -> i16 {
                -1
            }
        }

        $(
            impl Into<$name> for $field_type {
                fn into(self) -> $name {
                    $name::$field_name(self)
                }
            }
        )*
    };
}

macro_rules! impl_sql {
    ($name: ident) => {
        impl ::diesel::types::FromSql<::diesel::types::Binary> for $name {
            fn from_sql(bytes: Option<&[u8]>)
                        -> ::std::result::Result<Self, Box<::std::error::Error>> {
                let mut buf = ::std::io::Cursor::new(try!(Vec::from_sql(bytes)));
                $name::deserialize(&mut buf).map_err(|e| Box::new(e) as Box<::std::error::Error>)
            }
        }

        impl ::diesel::types::ToSql<::diesel::types::Binary> for $name {
            fn to_sql<W: ::std::io::Write>(&self, out: &mut W)
                                           -> ::std::result::Result<::diesel::types::IsNull,
                                                                    Box<::std::error::Error>> {
                use diesel::types::{ToSql, Binary};
                let mut buf = Vec::new();
                self.serialize(&mut buf).unwrap();
                ToSql::<Binary>::to_sql(&buf, out)
            }
        }

        impl ::diesel::expression::AsExpression<::diesel::types::Binary> for $name {
            type Expression = ::diesel::expression::bound::Bound<::diesel::types::Binary, Self>;

            fn as_expression(self) -> Self::Expression {
                ::diesel::expression::bound::Bound::new(self)
            }
        }

        impl<'a> ::diesel::expression::AsExpression<::diesel::types::Binary> for &'a $name {
            type Expression = ::diesel::expression::bound::Bound<::diesel::types::Binary, Self>;

            fn as_expression(self) -> Self::Expression {
                ::diesel::expression::bound::Bound::new(self)
            }
        }
    }
}

macro_rules! impl_type {
    ($name: ident, $id: expr) => {
        #[derive(Clone, Debug)]
        pub struct $name;

        impl Protocol for $name {
            fn deserialize<R: Read>(_: &mut R) -> Result<$name> {
                Ok($name)
            }

            fn serialize<W: Write>(&self, _: &mut W) -> Result<()> {
                Ok(())
            }

            fn id() -> i16 {
                $id
            }
        }

        impl_sql!($name);
    };

    ($name: ident, $id: expr, $($field_name: ident | $field_type: ty),*) => {

        #[derive(Clone, Debug)]
        pub struct $name {
            $(
                pub $field_name: $field_type,
            )*
        }

        impl Protocol for $name {
            fn deserialize<R: Read>(rdr: &mut R) -> Result<$name> {
                let mut flag = 0;
                let mut offset = 0;
                $(
                    let $field_name = try!(<$field_type as Protocol>
                        ::_deserialize(rdr, &mut flag, &mut offset));
                )*
                Ok($name {
                    $(
                        $field_name: $field_name,
                    )*
                })
            }

            fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
                let mut flag = None;
                let mut offset = 0;
                $(
                    try!(self
                        .$field_name
                        ._serialize(wtr, &mut flag, &mut offset));
                )*
                if let Some(byte) = flag {
                    use $crate::io::WriteExt;
                    try!(wtr.write_u8(byte));
                }
                Ok(())
            }

            fn id() -> i16 {
                $id
            }
        }

        impl_sql!($name);
    };
}


macro_rules! impl_primitive {
    ($t: ty, $read: ident, $write: ident) => {
        impl Protocol for $t {
            fn deserialize<R: Read>(rdr: &mut R) -> Result<$t> {
                rdr.$read()
            }

            fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
                wtr.$write(*self)
            }

            fn id() -> i16 {
                -1
            }
        }
    };
}

macro_rules! impl_var {
    ($p: path, $read: ident, $write: ident) => {
        impl Protocol for $p {
            fn deserialize<R: Read>(rdr: &mut R) -> Result<$p> {
                rdr.$read().map($p)
            }

            fn serialize<W: Write>(&self, wtr: &mut W) -> Result<()> {
                wtr.$write(self.0)
            }

            fn id() -> i16 {
                -1
            }
        }
    };
}

mod protocol;
pub mod debug;
pub mod io;
pub mod enums;
pub mod variants;
pub mod messages;
pub mod types;
pub mod holomorph;

pub use protocol::*;