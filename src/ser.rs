use indexmap::IndexMap;
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{ser, Serialize};

use crate::{Error, Value};

/// Convert `T: Serialize` into [`Value`].
///
/// # Examples
///
/// ```
/// use serde_bridge::{into_value, Value};
/// # use anyhow::Result;
/// # fn main() -> Result<()>{
/// let v = into_value(true)?;
/// # assert_eq!(v, Value::Bool(true));
/// # Ok(())
/// # }
/// ```
pub fn into_value(v: impl Serialize) -> Result<Value, Error> {
    v.serialize(Serializer)
}

/// Convert `T: Serialize` into [`Value`].
///
/// # Examples
///
/// ```
/// use serde_bridge::{IntoValue, Value};
/// # use anyhow::Result;
/// # fn main() -> Result<()>{
/// let v = true.into_value()?;
/// # assert_eq!(v, Value::Bool(true));
/// # Ok(())
/// # }
/// ```
pub trait IntoValue {
    fn into_value(self) -> Result<Value, Error>;
}

impl<T> IntoValue for T
where
    T: Serialize,
{
    fn into_value(self) -> Result<Value, Error> {
        into_value(self)
    }
}

/// Implement transparent [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) for [`Value`].
///
/// `Serialize` on `Value` that converted from `T` will be the same with `T`.
impl serde::Serialize for Value {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Bool(v) => s.serialize_bool(*v),
            Value::I8(v) => s.serialize_i8(*v),
            Value::I16(v) => s.serialize_i16(*v),
            Value::I32(v) => s.serialize_i32(*v),
            Value::I64(v) => s.serialize_i64(*v),
            Value::I128(v) => s.serialize_i128(*v),
            Value::U8(v) => s.serialize_u8(*v),
            Value::U16(v) => s.serialize_u16(*v),
            Value::U32(v) => s.serialize_u32(*v),
            Value::U64(v) => s.serialize_u64(*v),
            Value::U128(v) => s.serialize_u128(*v),
            Value::F32(v) => s.serialize_f32(*v),
            Value::F64(v) => s.serialize_f64(*v),
            Value::Char(v) => s.serialize_char(*v),
            Value::Str(v) => s.serialize_str(v),
            Value::Bytes(v) => s.serialize_bytes(v),
            Value::None => s.serialize_none(),
            Value::Some(v) => s.serialize_some(v),
            Value::Unit => s.serialize_unit(),
            Value::UnitStruct(name) => s.serialize_unit_struct(name),
            Value::UnitVariant {
                name,
                variant_index,
                variant,
            } => s.serialize_unit_variant(name, *variant_index, variant),
            Value::NewtypeStruct(name, value) => s.serialize_newtype_struct(name, value),
            Value::NewtypeVariant {
                name,
                variant_index,
                variant,
                value,
            } => s.serialize_newtype_variant(name, *variant_index, variant, value),
            Value::Seq(v) => {
                let mut seq = s.serialize_seq(Some(v.len()))?;
                for i in v {
                    seq.serialize_element(i)?;
                }
                seq.end()
            }
            Value::Tuple(v) => {
                let mut tuple = s.serialize_tuple(v.len())?;
                for i in v {
                    tuple.serialize_element(i)?;
                }
                tuple.end()
            }
            Value::TupleStruct(name, fields) => {
                let mut se = s.serialize_tuple_struct(name, fields.len())?;
                for i in fields {
                    se.serialize_field(i)?;
                }
                se.end()
            }
            Value::TupleVariant {
                name,
                variant_index,
                variant,
                fields,
            } => {
                let mut se =
                    s.serialize_tuple_variant(name, *variant_index, variant, fields.len())?;
                for i in fields {
                    se.serialize_field(i)?;
                }
                se.end()
            }
            Value::Map(map) => {
                let mut se = s.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    se.serialize_entry(k, v)?;
                }
                se.end()
            }
            Value::Struct(name, fields) => {
                let mut se = s.serialize_struct(name, fields.len())?;
                for (k, v) in fields {
                    se.serialize_field(k, v)?;
                }
                se.end()
            }
            Value::StructVariant {
                name,
                variant_index,
                variant,
                fields,
            } => {
                let mut se =
                    s.serialize_struct_variant(name, *variant_index, variant, fields.len())?;
                for (k, v) in fields {
                    se.serialize_field(k, v)?;
                }
                se.end()
            }
        }
    }
}

struct Serializer;

impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = SeqSerializer;
    type SerializeTuple = TupleSerializer;
    type SerializeTupleStruct = TupleStructSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Str(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bytes(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::None)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Some(Box::new(value.serialize(Serializer)?)))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitStruct(name))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitVariant {
            name,
            variant_index,
            variant,
        })
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::NewtypeStruct(
            name,
            Box::new(value.serialize(Serializer)?),
        ))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::NewtypeVariant {
            name,
            variant_index,
            variant,
            value: Box::new(value.serialize(Serializer)?),
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer::new(len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(TupleSerializer::new(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(TupleStructSerializer::new(name, len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer::new(
            name,
            variant_index,
            variant,
            len,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(len))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer::new(name, len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer::new(
            name,
            variant_index,
            variant,
            len,
        ))
    }
}

struct SeqSerializer {
    elements: Vec<Value>,
}

impl SeqSerializer {
    pub fn new(len: Option<usize>) -> Self {
        Self {
            elements: Vec::with_capacity(len.unwrap_or_default()),
        }
    }
}

impl ser::SerializeSeq for SeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.elements.push(value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Seq(self.elements))
    }
}

struct TupleSerializer {
    elements: Vec<Value>,
}

impl TupleSerializer {
    pub fn new(len: usize) -> Self {
        Self {
            elements: Vec::with_capacity(len),
        }
    }
}

impl ser::SerializeTuple for TupleSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.elements.push(value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Tuple(self.elements))
    }
}

struct TupleStructSerializer {
    name: &'static str,
    fields: Vec<Value>,
}

impl TupleStructSerializer {
    pub fn new(name: &'static str, len: usize) -> Self {
        Self {
            name,
            fields: Vec::with_capacity(len),
        }
    }
}

impl ser::SerializeTupleStruct for TupleStructSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.fields.push(value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleStruct(self.name, self.fields))
    }
}

struct TupleVariantSerializer {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    fields: Vec<Value>,
}

impl TupleVariantSerializer {
    pub fn new(name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Self {
        Self {
            name,
            variant_index,
            variant,
            fields: Vec::with_capacity(len),
        }
    }
}

impl ser::SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.fields.push(value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleVariant {
            name: self.name,
            variant_index: self.variant_index,
            variant: self.variant,
            fields: self.fields,
        })
    }
}

struct MapSerializer {
    cache_key: Option<Value>,
    entries: IndexMap<Value, Value>,
}

impl MapSerializer {
    pub fn new(len: Option<usize>) -> Self {
        Self {
            cache_key: None,
            entries: IndexMap::with_capacity(len.unwrap_or_default()),
        }
    }
}

impl ser::SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        debug_assert!(
            self.cache_key.is_none(),
            "value for the last entry is missing"
        );
        self.cache_key = Some(key.serialize(Serializer)?);

        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let key = self
            .cache_key
            .take()
            .expect("key for current entry is missing");
        self.entries.insert(key, value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.entries))
    }
}

struct StructSerializer {
    name: &'static str,
    fields: IndexMap<&'static str, Value>,
}

impl StructSerializer {
    pub fn new(name: &'static str, len: usize) -> Self {
        Self {
            name,
            fields: IndexMap::with_capacity(len),
        }
    }
}

impl ser::SerializeStruct for StructSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.fields.insert(key, value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct(self.name, self.fields))
    }
}

struct StructVariantSerializer {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    fields: IndexMap<&'static str, Value>,
}

impl StructVariantSerializer {
    pub fn new(name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Self {
        Self {
            name,
            variant_index,
            variant,
            fields: IndexMap::with_capacity(len),
        }
    }
}

impl ser::SerializeStructVariant for StructVariantSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.fields.insert(key, value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::StructVariant {
            name: self.name,
            variant_index: self.variant_index,
            variant: self.variant,
            fields: self.fields,
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use indexmap::indexmap;

    use super::*;

    #[derive(serde::Serialize)]
    struct TestStruct {
        a: bool,
        b: i32,
        c: u64,
        d: String,
        e: f64,
    }

    #[test]
    fn test_to_value() {
        assert_eq!(into_value(128).expect("must success"), Value::I32(128));
        assert_eq!(into_value(128_u64).expect("must success"), Value::U64(128));

        assert_eq!(
            into_value(TestStruct {
                a: true,
                b: 1,
                c: 2,
                d: "Hello, World!".to_string(),
                e: 4.5
            })
            .expect("must success"),
            Value::Struct(
                "TestStruct",
                indexmap! {
                    "a" => Value::Bool(true),
                    "b" => Value::I32(1),
                    "c" => Value::U64(2),
                    "d" => Value::Str("Hello, World!".to_string()),
                    "e" => Value::F64(4.5)
                }
            )
        )
    }

    #[test]
    fn test_serialize() -> Result<()> {
        let raw = TestStruct {
            a: true,
            b: 1,
            c: 2,
            d: "Hello, World!".to_string(),
            e: 4.5,
        };
        let value = Value::Struct(
            "TestStruct",
            indexmap! {
                "a" => Value::Bool(true),
                "b" => Value::I32(1),
                "c" => Value::U64(2),
                "d" => Value::Str("Hello, World!".to_string()),
                "e" => Value::F64(4.5)
            },
        );

        assert_eq!(serde_json::to_string(&raw)?, serde_json::to_string(&value)?);

        Ok(())
    }
}
