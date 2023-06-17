use std::hash::{Hash, Hasher};

use indexmap::IndexMap;

/// Value is the internal represents of serde's data format.
///
/// Value is the one-to-one map to [serde's data format](https://serde.rs/data-model.html).
/// Theoretically, `Value` can be converted from/to any serde format.
///
/// `Value` is a inter data represents which means:
///
/// `Value` should be constructed or consumed by `into_value` or `from_value`.
///
/// - `t.into_value()` -> `Value`
/// - `into_value(t)` -> `Value`
/// - `T::from_value(v)` -> `T`
/// - `from_value(v)` -> `T`
///
/// `Value` also implements [`serde::Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) and [`serde::Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html).
/// `Serialize` and `Deserialize` on `Value` that converted from `T` will be the same with `T`.
///
/// # Examples
///
/// ## Conversion between `T` and `Value`
///
/// ```
/// use anyhow::Result;
/// use serde_bridge::{from_value, into_value, FromValue, IntoValue, Value};
///
/// fn main() -> Result<()> {
///     let v = bool::from_value(Value::Bool(true))?;
///     assert!(v);
///
///     let v: bool = from_value(Value::Bool(true))?;
///     assert!(v);
///
///     let v = true.into_value()?;
///     assert_eq!(v, Value::Bool(true));
///
///     let v = into_value(true)?;
///     assert_eq!(v, Value::Bool(true));
///
///     Ok(())
/// }
/// ```
///
/// ## Transparent Serialize and Deserialize
///
/// ```
/// use anyhow::Result;
/// use serde_bridge::{from_value, into_value, FromValue, IntoValue, Value};
/// use serde_json;
///
/// fn main() -> Result<()> {
///     let raw = serde_json::to_string(&true)?;
///     let value = serde_json::to_string(&true.into_value()?)?;
///     assert_eq!(raw, value);
///
///     let raw: bool = serde_json::from_str("true")?;
///     let value: Value = serde_json::from_str("true")?;
///     assert_eq!(raw, bool::from_value(value)?);
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// primitive types for `bool`: `false`/`true`
    Bool(bool),
    /// primitive types for `i8`
    I8(i8),
    /// primitive types for `i16`
    I16(i16),
    /// primitive types for `i32`
    I32(i32),
    /// primitive types for `i64`
    I64(i64),
    /// primitive types for `i128`
    I128(i128),
    /// primitive types for `u8`
    U8(u8),
    /// primitive types for `u16`
    U16(u16),
    /// primitive types for `u32`
    U32(u32),
    /// primitive types for `u64`
    U64(u64),
    /// primitive types for `u128`
    U128(u128),
    /// primitive types for `f32`
    F32(f32),
    /// primitive types for `f64`
    F64(f64),
    /// primitive types for `char`
    Char(char),
    /// string type
    ///
    /// UTF-8 bytes with a length and no null terminator. May contain 0-bytes.
    Str(String),
    /// byte array
    ///
    /// Similar to strings, during deserialization byte arrays can be transient, owned, or borrowed.
    Bytes(Vec<u8>),
    /// `None` part of an `Option`
    None,
    /// `Some` part of an `Option`
    ///
    /// # Note
    ///
    /// We use `Box` here to workaround recursive data type.
    Some(Box<Value>),
    /// The type of `()` in Rust.
    ///
    /// It represents an anonymous value containing no data.
    Unit,
    /// For example `struct Unit` or `PhantomData<T>`.
    ///
    /// It represents a named value containing no data.
    UnitStruct(&'static str),
    /// For example the `E::A` and `E::B` in `enum E { A, B }`.
    UnitVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    },
    /// For example struct `Millimeters(u8)`.
    NewtypeStruct(&'static str, Box<Value>),
    /// For example the `E::N` in `enum E { N(u8) }`.
    ///
    /// # Note
    ///
    /// We use `Box` here to workaround recursive data type.
    NewtypeVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: Box<Value>,
    },
    /// A variably sized heterogeneous sequence of values, for example `Vec<T>` or `HashSet<T>`
    Seq(Vec<Value>),
    /// A statically sized heterogeneous sequence of values for which the length will be known at deserialization time without looking at the serialized data.
    ///
    /// For example `(u8,)` or `(String, u64, Vec<T>)` or `[u64; 10]`.
    Tuple(Vec<Value>),
    /// A named tuple, for example `struct Rgb(u8, u8, u8)`.
    TupleStruct(&'static str, Vec<Value>),
    /// For example the `E::T` in `enum E { T(u8, u8) }`.
    TupleVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        fields: Vec<Value>,
    },
    /// A variably sized heterogeneous key-value pairing, for example `BTreeMap<K, V>`
    Map(IndexMap<Value, Value>),
    /// A statically sized heterogeneous key-value pairing in which the keys are compile-time
    /// constant strings and will be known at deserialization time without looking at the
    /// serialized data.
    ///
    /// For example `struct S { r: u8, g: u8, b: u8 }`.
    Struct(&'static str, IndexMap<&'static str, Value>),
    /// For example the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`.
    StructVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        fields: IndexMap<&'static str, Value>,
    },
}

impl Eq for Value {}

/// Implement Hash for Value so that we can use value as hash key.
///
/// ## Notes
///
/// Not all variants supports hash.
///
/// ## FIXME
///
/// does this implementation correct?
#[allow(clippy::derived_hash_with_manual_eq)]
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Write current enum discriminant into state.
        std::mem::discriminant(self).hash(state);
        match self {
            Value::Bool(v) => v.hash(state),
            Value::I8(v) => v.hash(state),
            Value::I16(v) => v.hash(state),
            Value::I32(v) => v.hash(state),
            Value::I64(v) => v.hash(state),
            Value::I128(v) => v.hash(state),
            Value::U8(v) => v.hash(state),
            Value::U16(v) => v.hash(state),
            Value::U32(v) => v.hash(state),
            Value::U64(v) => v.hash(state),
            Value::U128(v) => v.hash(state),
            Value::F32(_) => panic!("f32 is not hashable"),
            Value::F64(_) => panic!("f64 is not hashable"),
            Value::Char(v) => v.hash(state),
            Value::Str(v) => v.hash(state),
            Value::Bytes(v) => v.hash(state),
            Value::None => {}
            Value::Some(v) => v.hash(state),
            Value::Unit => {}
            Value::UnitStruct(v) => v.hash(state),
            Value::UnitVariant {
                name,
                variant_index,
                variant,
            } => {
                name.hash(state);
                variant_index.hash(state);
                variant.hash(state);
            }
            Value::NewtypeStruct(name, value) => {
                name.hash(state);
                value.hash(state);
            }
            Value::NewtypeVariant {
                name,
                variant_index,
                variant,
                value,
            } => {
                name.hash(state);
                variant_index.hash(state);
                variant.hash(state);
                value.hash(state);
            }
            Value::Seq(v) => v.hash(state),
            Value::Tuple(v) => v.hash(state),
            Value::TupleStruct(name, fields) => {
                name.hash(state);
                fields.hash(state);
            }
            Value::TupleVariant {
                name,
                variant_index,
                variant,
                fields,
            } => {
                name.hash(state);
                variant_index.hash(state);
                variant.hash(state);
                fields.hash(state);
            }
            Value::Map(v) => {
                for e in v {
                    e.hash(state)
                }
            }
            Value::Struct(name, fields) => {
                name.hash(state);
                for e in fields {
                    e.hash(state)
                }
            }
            Value::StructVariant {
                name,
                variant_index,
                variant,
                fields,
            } => {
                name.hash(state);
                variant_index.hash(state);
                variant.hash(state);
                for e in fields {
                    e.hash(state)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_size() {
        println!("Size is {}", std::mem::size_of::<Value>());
    }
}
