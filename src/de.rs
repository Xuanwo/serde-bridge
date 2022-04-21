use std::fmt::Formatter;
use std::vec::IntoIter;

use anyhow::anyhow;
use indexmap::IndexMap;
use serde::de::{DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{de, Deserialize};

use crate::{Error, Value};

/// Convert [`Value`] into `T: DeserializeOwned`.
///
/// # Examples
///
/// ```
/// use serde_bridge::{from_value, Value};
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// let v: bool = from_value(Value::Bool(true))?;
/// # assert!(v);
/// # Ok(())
/// # }
/// ```
pub fn from_value<T: DeserializeOwned>(v: Value) -> Result<T, Error> {
    T::deserialize(Deserializer(v))
}

/// Convert [`Value`] into `T: DeserializeOwned`.
///
/// # Examples
///
/// ```
/// use serde_bridge::{FromValue, Value};
/// # use anyhow::Result;
/// # fn main() -> Result<()>{
/// let v = bool::from_value(Value::Bool(true))?;
/// # assert!(v);
/// # Ok(())
/// # }
/// ```
pub trait FromValue {
    fn from_value(v: Value) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T> FromValue for T
where
    T: DeserializeOwned,
{
    fn from_value(v: Value) -> Result<Self, Error> {
        from_value(v)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "expecting visitor")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::I8(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::I16(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::I32(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::I64(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::U8(v))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::U16(v))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::U32(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::U64(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::F32(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::F64(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Char(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Str(v.to_string()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Str(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Str(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Bytes(v.to_vec()))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Bytes(v.to_vec()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Bytes(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::None)
    }

    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Value::Some(Box::new(d.deserialize_any(ValueVisitor)?)))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Unit)
    }

    fn visit_newtype_struct<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Value::NewtypeStruct(
            "",
            Box::new(d.deserialize_any(ValueVisitor)?),
        ))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(v) = seq.next_element()? {
            vec.push(v);
        }
        Ok(Value::Seq(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut im = IndexMap::new();
        while let Some((k, v)) = map.next_entry()? {
            im.insert(k, v);
        }
        Ok(Value::Map(im))
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_any(ValueVisitor)
    }
}

struct Deserializer(Value);

impl<'de> serde::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &self.0 {
            Value::Bool(_) => self.deserialize_bool(vis),
            Value::I8(_) => self.deserialize_i8(vis),
            Value::I16(_) => self.deserialize_i16(vis),
            Value::I32(_) => self.deserialize_i32(vis),
            Value::I64(_) => self.deserialize_i64(vis),
            Value::I128(_) => self.deserialize_i128(vis),
            Value::U8(_) => self.deserialize_u8(vis),
            Value::U16(_) => self.deserialize_u16(vis),
            Value::U32(_) => self.deserialize_u32(vis),
            Value::U64(_) => self.deserialize_u64(vis),
            Value::U128(_) => self.deserialize_u128(vis),
            Value::F32(_) => self.deserialize_f32(vis),
            Value::F64(_) => self.deserialize_f64(vis),
            Value::Char(_) => self.deserialize_char(vis),
            Value::Str(_) => self.deserialize_string(vis),
            Value::Bytes(_) => self.deserialize_byte_buf(vis),
            Value::None => self.deserialize_option(vis),
            Value::Some(_) => self.deserialize_option(vis),
            Value::Unit => self.deserialize_unit(vis),
            Value::Map(_) => self.deserialize_map(vis),
            Value::Seq(_) => self.deserialize_seq(vis),
            Value::Struct(_, _) => self.deserialize_map(vis),
            v => unimplemented!("deserialize_any for {:?}", v),
        }
    }

    fn deserialize_bool<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Bool(v) => vis.visit_bool(v),
            v => Err(Error(anyhow!("invalid type: {:?}", v))),
        }
    }

    fn deserialize_i8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_i8(v),
            Value::I16(v) => vis.visit_i8(i8::try_from(v)?),
            Value::I32(v) => vis.visit_i8(i8::try_from(v)?),
            Value::I64(v) => vis.visit_i8(i8::try_from(v)?),
            Value::I128(v) => vis.visit_i8(i8::try_from(v)?),
            Value::U8(v) => vis.visit_i8(i8::try_from(v)?),
            Value::U16(v) => vis.visit_i8(i8::try_from(v)?),
            Value::U32(v) => vis.visit_i8(i8::try_from(v)?),
            Value::U64(v) => vis.visit_i8(i8::try_from(v)?),
            Value::U128(v) => vis.visit_i8(i8::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}, expect i8", v))),
        }
    }

    fn deserialize_i16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_i16(i16::from(v)),
            Value::I16(v) => vis.visit_i16(v),
            Value::I32(v) => vis.visit_i16(i16::try_from(v)?),
            Value::I64(v) => vis.visit_i16(i16::try_from(v)?),
            Value::I128(v) => vis.visit_i16(i16::try_from(v)?),
            Value::U8(v) => vis.visit_i16(i16::from(v)),
            Value::U16(v) => vis.visit_i16(i16::try_from(v)?),
            Value::U32(v) => vis.visit_i16(i16::try_from(v)?),
            Value::U64(v) => vis.visit_i16(i16::try_from(v)?),
            Value::U128(v) => vis.visit_i16(i16::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}, expect i16", v))),
        }
    }

    fn deserialize_i32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_i32(i32::from(v)),
            Value::I16(v) => vis.visit_i32(i32::from(v)),
            Value::I32(v) => vis.visit_i32(v),
            Value::I64(v) => vis.visit_i32(i32::try_from(v)?),
            Value::I128(v) => vis.visit_i32(i32::try_from(v)?),
            Value::U8(v) => vis.visit_i32(i32::from(v)),
            Value::U16(v) => vis.visit_i32(i32::from(v)),
            Value::U32(v) => vis.visit_i32(i32::try_from(v)?),
            Value::U64(v) => vis.visit_i32(i32::try_from(v)?),
            Value::U128(v) => vis.visit_i32(i32::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}", v))),
        }
    }

    fn deserialize_i64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_i64(i64::from(v)),
            Value::I16(v) => vis.visit_i64(i64::from(v)),
            Value::I32(v) => vis.visit_i64(i64::from(v)),
            Value::I64(v) => vis.visit_i64(v),
            Value::I128(v) => vis.visit_i64(i64::try_from(v)?),
            Value::U8(v) => vis.visit_i64(i64::from(v)),
            Value::U16(v) => vis.visit_i32(i32::from(v)),
            Value::U32(v) => vis.visit_i64(i64::from(v)),
            Value::U64(v) => vis.visit_i64(i64::try_from(v)?),
            Value::U128(v) => vis.visit_i64(i64::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}, expect i64", v))),
        }
    }

    fn deserialize_u8<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_u8(u8::try_from(v)?),
            Value::I16(v) => vis.visit_u8(u8::try_from(v)?),
            Value::I32(v) => vis.visit_u8(u8::try_from(v)?),
            Value::I64(v) => vis.visit_u8(u8::try_from(v)?),
            Value::I128(v) => vis.visit_u8(u8::try_from(v)?),
            Value::U8(v) => vis.visit_u8(v),
            Value::U16(v) => vis.visit_u8(u8::try_from(v)?),
            Value::U32(v) => vis.visit_u8(u8::try_from(v)?),
            Value::U64(v) => vis.visit_u8(u8::try_from(v)?),
            Value::U128(v) => vis.visit_u8(u8::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}, expect u8", v))),
        }
    }

    fn deserialize_u16<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_u16(u16::try_from(v)?),
            Value::I16(v) => vis.visit_u16(u16::try_from(v)?),
            Value::I32(v) => vis.visit_u16(u16::try_from(v)?),
            Value::I64(v) => vis.visit_u16(u16::try_from(v)?),
            Value::I128(v) => vis.visit_u16(u16::try_from(v)?),
            Value::U8(v) => vis.visit_u16(u16::from(v)),
            Value::U16(v) => vis.visit_u16(v),
            Value::U32(v) => vis.visit_u16(u16::try_from(v)?),
            Value::U64(v) => vis.visit_u16(u16::try_from(v)?),
            Value::U128(v) => vis.visit_u16(u16::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}, expect u16", v))),
        }
    }

    fn deserialize_u32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_u32(u32::try_from(v)?),
            Value::I16(v) => vis.visit_u32(u32::try_from(v)?),
            Value::I32(v) => vis.visit_u32(u32::try_from(v)?),
            Value::I64(v) => vis.visit_u32(u32::try_from(v)?),
            Value::I128(v) => vis.visit_u32(u32::try_from(v)?),
            Value::U8(v) => vis.visit_u32(u32::from(v)),
            Value::U16(v) => vis.visit_u32(u32::from(v)),
            Value::U32(v) => vis.visit_u32(v),
            Value::U64(v) => vis.visit_u32(u32::try_from(v)?),
            Value::U128(v) => vis.visit_u32(u32::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}, expect u32", v))),
        }
    }

    fn deserialize_u64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::I8(v) => vis.visit_u64(u64::try_from(v)?),
            Value::I16(v) => vis.visit_u64(u64::try_from(v)?),
            Value::I32(v) => vis.visit_u64(u64::try_from(v)?),
            Value::I64(v) => vis.visit_u64(u64::try_from(v)?),
            Value::I128(v) => vis.visit_u64(u64::try_from(v)?),
            Value::U8(v) => vis.visit_u64(u64::from(v)),
            Value::U16(v) => vis.visit_u64(u64::from(v)),
            Value::U32(v) => vis.visit_u64(u64::from(v)),
            Value::U64(v) => vis.visit_u64(v),
            Value::U128(v) => vis.visit_u64(u64::try_from(v)?),
            v => Err(Error(anyhow!("invalid type: {:?}, expect u64", v))),
        }
    }

    fn deserialize_f32<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::F32(v) => vis.visit_f32(v),
            Value::F64(v) => vis.visit_f32(v as f32),
            v => Err(Error(anyhow!("invalid type: {:?}, expect f32", v))),
        }
    }

    fn deserialize_f64<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::F32(v) => vis.visit_f64(f64::from(v)),
            Value::F64(v) => vis.visit_f64(v),
            v => Err(Error(anyhow!("invalid type: {:?}, expect f64", v))),
        }
    }

    fn deserialize_char<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Char(v) => vis.visit_char(v),
            v => Err(Error(anyhow!("invalid type: {:?}, expect char", v))),
        }
    }

    fn deserialize_str<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Str(v) => vis.visit_string(v),
            v => Err(Error(anyhow!("invalid type: {:?}, expect str", v))),
        }
    }

    fn deserialize_string<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Str(v) => vis.visit_string(v),
            v => Err(Error(anyhow!("invalid type: {:?}, expect string", v))),
        }
    }

    fn deserialize_bytes<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Bytes(v) => vis.visit_byte_buf(v),
            v => Err(Error(anyhow!("invalid type: {:?}, expect bytes", v))),
        }
    }

    fn deserialize_byte_buf<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Bytes(v) => vis.visit_byte_buf(v),
            v => Err(Error(anyhow!("invalid type: {:?}, expect bytes_buf", v))),
        }
    }

    fn deserialize_option<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::None => vis.visit_none(),
            Value::Some(v) => vis.visit_some(Deserializer(*v)),
            v => Err(Error(anyhow!("invalid type: {:?}, expect option", v))),
        }
    }

    fn deserialize_unit<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Unit => vis.visit_unit(),
            v => Err(Error(anyhow!("invalid type: {:?}, expect unit", v))),
        }
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::UnitStruct(vn) if vn == name => vis.visit_unit(),
            v => Err(Error(anyhow!("invalid type: {:?}, expect unit struct", v))),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::NewtypeStruct(vn, vv) if vn == name => {
                vis.visit_newtype_struct(Deserializer(*vv))
            }
            v => Err(Error(anyhow!(
                "invalid type: {:?}, expect newtype struct",
                v
            ))),
        }
    }

    fn deserialize_seq<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Tuple(v) => vis.visit_seq(SeqAccessor::new(v)),
            Value::Seq(v) => vis.visit_seq(SeqAccessor::new(v)),
            v => Err(Error(anyhow!("invalid type: {:?}, expect seq", v))),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Tuple(v) if len == v.len() => vis.visit_seq(SeqAccessor::new(v)),
            Value::Seq(v) if len == v.len() => vis.visit_seq(SeqAccessor::new(v)),
            v => Err(Error(anyhow!("invalid type: {:?}, expect tuple", v))),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::TupleStruct(vn, vf) if name == vn && len == vf.len() => {
                vis.visit_seq(SeqAccessor::new(vf))
            }
            v => Err(Error(anyhow!("invalid type: {:?}, expect tuple struct", v))),
        }
    }

    fn deserialize_map<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Map(v) => vis.visit_map(MapAccessor::new(v)),
            v => Err(Error(anyhow!("invalid type: {:?}, expect map", v))),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Value::Struct(vn, mut vf) if vn == name => {
                let mut vs = Vec::with_capacity(fields.len());
                for key in fields {
                    // Use `remove` instead of `get` & `clone` here.
                    // - As serde will make sure to not access the same field twice.
                    // - The order of key is not needed to preserve during deserialize.
                    match vf.remove(key) {
                        Some(v) => vs.push(v),
                        None => return Err(Error(anyhow!("field not exist"))),
                    }
                }
                vis.visit_seq(SeqAccessor::new(vs))
            }
            Value::Map(fields) => vis.visit_map(MapAccessor::new(fields)),
            v => Err(Error(anyhow!("invalid type: {:?}, expect struct", v))),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        vis.visit_enum(EnumAccessor::new(name, variants, self.0))
    }

    fn deserialize_identifier<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(vis)
    }

    fn deserialize_ignored_any<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(vis)
    }
}

struct SeqAccessor {
    elements: IntoIter<Value>,
}

impl SeqAccessor {
    fn new(elements: Vec<Value>) -> Self {
        Self {
            elements: elements.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccessor {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.elements.next() {
            None => Ok(None),
            Some(v) => Ok(Some(seed.deserialize(Deserializer(v))?)),
        }
    }
}

struct MapAccessor {
    cache_value: Option<Value>,
    entries: indexmap::map::IntoIter<Value, Value>,
}

impl MapAccessor {
    fn new(entries: IndexMap<Value, Value>) -> Self {
        Self {
            cache_value: None,
            entries: entries.into_iter(),
        }
    }
}
impl<'de> de::MapAccess<'de> for MapAccessor {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        debug_assert!(
            self.cache_value.is_none(),
            "value for the last entry is not deserialized"
        );

        match self.entries.next() {
            None => Ok(None),
            Some((k, v)) => {
                self.cache_value = Some(v);
                Ok(Some(seed.deserialize(Deserializer(k))?))
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self
            .cache_value
            .take()
            .expect("value for current entry is missing");
        seed.deserialize(Deserializer(value))
    }
}

struct EnumAccessor {
    name: &'static str,
    variants: &'static [&'static str],
    value: Value,
}

impl EnumAccessor {
    fn new(name: &'static str, variants: &'static [&'static str], value: Value) -> Self {
        Self {
            name,
            variants,
            value,
        }
    }
}

impl<'de> de::EnumAccess<'de> for EnumAccessor {
    type Error = Error;
    type Variant = VariantAccessor;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = match &self.value {
            Value::UnitVariant {
                name: vn,
                variant_index: vvi,
                variant: vv,
            } if &self.name == vn && &self.variants[*vvi as usize] == vv => {
                seed.deserialize(Deserializer(Value::Str(vv.to_string())))?
            }
            Value::TupleVariant {
                name: vn,
                variant_index: vvi,
                variant: vv,
                ..
            } if &self.name == vn && &self.variants[*vvi as usize] == vv => {
                seed.deserialize(Deserializer(Value::Str(vv.to_string())))?
            }
            Value::StructVariant {
                name: vn,
                variant_index: vvi,
                variant: vv,
                ..
            } if &self.name == vn && &self.variants[*vvi as usize] == vv => {
                seed.deserialize(Deserializer(Value::Str(vv.to_string())))?
            }
            Value::NewtypeVariant {
                name: vn,
                variant_index: vvi,
                variant: vv,
                ..
            } if &self.name == vn && &self.variants[*vvi as usize] == vv => {
                seed.deserialize(Deserializer(Value::Str(vv.to_string())))?
            }
            _ => return Err(Error(anyhow!("invalid type"))),
        };

        Ok((value, VariantAccessor::new(self.value)))
    }
}

struct VariantAccessor {
    value: Value,
}

impl VariantAccessor {
    fn new(value: Value) -> Self {
        Self { value }
    }
}

impl<'de> de::VariantAccess<'de> for VariantAccessor {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Value::UnitVariant { .. } => Ok(()),
            _ => return Err(Error(anyhow!("invalid type"))),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Value::NewtypeVariant { value, .. } => Ok(seed.deserialize(Deserializer(*value))?),
            _ => return Err(Error(anyhow!("invalid type"))),
        }
    }

    fn tuple_variant<V>(self, len: usize, vis: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::TupleVariant { fields, .. } if len == fields.len() => {
                vis.visit_seq(SeqAccessor::new(fields))
            }
            _ => return Err(Error(anyhow!("invalid type"))),
        }
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        vis: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Struct(_, mut vf) => {
                let mut vs = Vec::with_capacity(fields.len());
                for key in fields {
                    // Use `remove` instead of `get` & `clone` here.
                    // - As serde will make sure to not access the same field twice.
                    // - The order of key is not needed to preserve during deserialize.
                    match vf.remove(key) {
                        Some(v) => vs.push(v),
                        None => return Err(Error(anyhow!("field not exist"))),
                    }
                }
                vis.visit_seq(SeqAccessor::new(vs))
            }
            _ => Err(Error(anyhow!("invalid type"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use indexmap::indexmap;

    use super::*;
    use crate::de::from_value;

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct TestStruct {
        a: bool,
        b: i32,
        c: u64,
        d: String,
        e: f64,
    }

    #[test]
    fn test_from_value() {
        let v: bool = from_value(Value::Bool(true)).expect("must success");
        assert!(v);

        let v: TestStruct = from_value(Value::Struct(
            "TestStruct",
            indexmap! {
                "a" => Value::Bool(true),
                "b" => Value::I32(1),
                "c" => Value::U64(2),
                "d" => Value::Str("Hello, World!".to_string()),
                "e" => Value::F64(4.5)
            },
        ))
        .expect("must success");
        assert_eq!(
            v,
            TestStruct {
                a: true,
                b: 1,
                c: 2,
                d: "Hello, World!".to_string(),
                e: 4.5
            }
        )
    }

    #[test]
    fn test_deserialize() -> Result<()> {
        let content = r#"{
            "a": true,
            "b": 1,
            "c": 2,
            "d": "Hello, World!",
            "e": 4.5
        }"#;
        let raw: TestStruct = serde_json::from_str(content)?;
        let value: Value = serde_json::from_str(content)?;
        println!("{:?}", value);

        assert_eq!(TestStruct::from_value(value)?, raw);

        Ok(())
    }
}
