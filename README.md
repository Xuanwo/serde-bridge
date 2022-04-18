# serde-bridge

Bridge between serde types

## Quick Start

```rust
use anyhow::Result;
use serde_bridge::{from_value, into_value, FromValue, IntoValue, Value};

fn main() -> Result<()> {
    let v = bool::from_value(Value::Bool(true))?;
    assert!(v);

    let v: bool = from_value(Value::Bool(true))?;
    assert!(v);

    let v = true.into_value()?;
    assert_eq!(v, Value::Bool(true));

    let v = into_value(true)?;
    assert_eq!(v, Value::Bool(true));

    Ok(())
}
```
