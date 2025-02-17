//! serde-bridge intends to be a bridge between different serde implementations.
//!
//! # Examples
//!
//! ```
//! use anyhow::Result;
//! use serde_bridge::{from_value, into_value, FromValue, IntoValue, Value};
//!
//! fn main() -> Result<()> {
//!     let v = bool::from_value(Value::Bool(true))?;
//!     assert!(v);
//!
//!     let v: bool = from_value(Value::Bool(true))?;
//!     assert!(v);
//!
//!     let v = true.into_value()?;
//!     assert_eq!(v, Value::Bool(true));
//!
//!     let v = into_value(true)?;
//!     assert_eq!(v, Value::Bool(true));
//!
//!     Ok(())
//! }
//! ```

mod value;
pub use value::Value;

mod de;
pub use de::{from_value, Deserializer, FromValue};

mod ser;
pub use ser::{into_value, IntoValue};

mod error;
use error::Error;
