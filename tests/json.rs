use std::collections::BTreeMap;

use anyhow::Result;
use indexmap::indexmap;
use serde::{Deserialize, Serialize};
use serde_bridge::{from_value, Value};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    a: bool,
    b: i32,
    c: u64,
    d: String,
    e: f64,
    f: Vec<u8>,
    g: [u16; 3],
    h: BTreeMap<String, f32>,
}

#[test]
fn test_to_json() -> Result<()> {
    let raw = TestStruct {
        a: true,
        b: 1,
        c: 2,
        d: "Hello, World!".to_string(),
        e: 3.4,
        f: vec![6, 7, 8, 9, 10],
        g: [11, 12, 13],
        h: BTreeMap::from([("a".to_string(), 10.1), ("b".to_string(), 11.3)]),
    };
    let value = Value::Struct(
        "TestStruct",
        indexmap! {
        "a" => Value::Bool(true),
        "b" => Value::I32(1),
        "c" => Value::U64(2),
        "d" => Value::Str("Hello, World!".to_string()),
        "e" => Value::F64(3.4),
        "f" => Value::Seq(vec![
            Value::U8(6),
            Value::U8(7),
            Value::U8(8),
            Value::U8(9),
            Value::U8(10),
        ]),
        "g" => Value::Tuple(vec![
            Value::U16(11),
            Value::U16(12),
            Value::U16(13),
        ]),
        "h" => Value::Map(
            indexmap! {
                Value::Str("a".to_string()) => Value::F32(10.1),
                Value::Str("b".to_string()) => Value::F32(11.3),
            }
        ),},
    );

    assert_eq!(serde_json::to_string(&raw)?, serde_json::to_string(&value)?);

    Ok(())
}

#[test]
fn test_from_json() -> Result<()> {
    let json = r#"{
        "a": true,
        "b": 1,
        "c": 2,
        "d": "Hello, World!",
        "e": 3.4,
        "f": [6,7,8,9,10],
        "g": [11,12,13],
        "h": {
            "a": 10.1,
            "b": 11.3
        }
    }"#;

    let raw: TestStruct = serde_json::from_str(json)?;
    let value: Value = serde_json::from_str(json)?;

    assert_eq!(raw, from_value(value)?);

    Ok(())
}
