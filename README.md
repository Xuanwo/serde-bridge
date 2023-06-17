# serde-bridge &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![chat]][discord]

[Build Status]: https://img.shields.io/github/actions/workflow/status/Xuanwo/serde-bridge/ci.yml?branch=main
[actions]: https://github.com/Xuanwo/serde-bridge/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/serde-bridge.svg
[crates.io]: https://crates.io/crates/serde-bridge
[chat]: https://img.shields.io/discord/1111711408875393035
[discord]: https://discord.gg/bmczSs2C69

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

## Contributing

Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Getting help

Submit [issues](https://github.com/Xuanwo/serde-bridge/issues/new/choose) for bug report or asking questions in [discussion](https://github.com/Xuanwo/serde-bridge/discussions/new?category=q-a).

## Acknowledgment

This project is highly inspired by [serde-value](https://github.com/arcnmx/serde-value)

#### License

<sup>
Licensed under <a href="./LICENSE">Apache License, Version 2.0</a>.
</sup>
