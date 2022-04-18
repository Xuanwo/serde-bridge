use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TestStruct {
    a: bool,
    b: i32,
    c: u64,
    d: String,
    e: f64,
    f: Vec<u8>,
    g: [u16; 3],
    h: HashMap<String, f32>,
}

#[test]
#[ignore]
fn test_from_json() -> Result<()> {
    todo!()
}
